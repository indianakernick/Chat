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

// TODO: Handle invalid group_id
pub async fn channel(group_id: db::GroupID, mut channel_id: db::ChannelID, session_id: db::SessionID, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let user = match db::session_user(pool.clone(), &session_id).await? {
        Some(user) => user,
        None => return Ok(Box::new(warp::redirect(
            format!("/login?redirect=/channel/{}/{}", group_id, channel_id)
                 .parse::<warp::http::Uri>().unwrap()
        )))
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
    let channel_name = match channel_list.iter().find(|c| c.channel_id == channel_id) {
        Some(channel) => channel.name.as_str(),
        None => {
            channel_id = channel_list[0].channel_id;
            channel_list[0].name.as_str()
        }
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
