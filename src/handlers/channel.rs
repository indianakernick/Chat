use askama::Template;
use serde::Serialize;
use crate::database as db;
use deadpool_postgres::Pool;

#[derive(Template)]
#[template(path = "channel.html")]
struct ChannelTemplate {
    title: String,
    group_id: db::GroupID,
    channel_id: db::ChannelID,
    user_id: db::UserID,
    user_info: String,
    group_list: String,
    channel_list: String,
    url: String,
}

fn ser_json<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value).unwrap().replace("</script>", "<\\/script>")
}

// TODO: Maybe we can do this better...
// Invalid channel takes the user to the first channel of the group
// Invalid group takes the user to the first group of the user
// Careful not to reveal any information to an attacker

pub async fn channel(group_id: db::GroupID, channel_id: Option<db::ChannelID>, session_id: db::SessionID, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let user = match db::session_user(pool.clone(), &session_id).await? {
        Some(user) => user,
        None => return Ok(Box::new(warp::redirect(match channel_id {
            Some(id) => format!("/login?redirect=/channel/{}/{}", group_id, id),
            None => format!("/login?redirect=/group/{}", group_id),
        }.parse::<warp::http::Uri>().unwrap())))
    };

    let (group_list, channel_list) = futures::future::join(
        db::group_list(pool.clone(), user.user_id),
        db::group_channels(pool.clone(), group_id)
    ).await;

    let group_list = group_list?;
    let channel_list = channel_list?;

    if group_list.is_empty() || channel_list.is_empty() {
        return Ok(Box::new(warp::http::StatusCode::NOT_FOUND));
    }

    let group_name = match group_list.iter().find(|g| g.group_id == group_id) {
        Some(group) => group.name.clone(),
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };
    let channel_id = match channel_id {
        Some(id) => id,
        None => channel_list[0].channel_id
    };
    let channel_name = match channel_list.iter().find(|c| c.channel_id == channel_id) {
        Some(channel) => channel.name.as_str(),
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };

    let user_info = db::AnonUser {
        name: user.name,
        picture: user.picture
    };

    Ok(Box::new(ChannelTemplate {
        title: group_name + "#" + channel_name,
        group_id,
        channel_id,
        user_id: user.user_id,
        user_info: ser_json(&user_info),
        group_list: ser_json(&group_list),
        channel_list: ser_json(&channel_list),
        url: format!("/channel/{}/{}", group_id, channel_id),
    }))
}
