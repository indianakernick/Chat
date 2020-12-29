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

pub async fn channel(mut group_id: db::GroupID, mut channel_id: db::ChannelID, session_id: db::SessionID, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    let user = match db::session_user(pool.clone(), &session_id).await? {
        Some(user) => user,
        None => return Ok(Box::new(warp::redirect(
            format!("/login?redirect=/channel/{}/{}", group_id, channel_id)
                 .parse::<warp::http::Uri>().unwrap()
        )))
    };

    let user_info = db::AnonUser {
        name: user.name,
        picture: user.picture
    };

    let group_list = db::user_groups(pool.clone(), user.user_id).await?;

    if group_list.is_empty() {
        return Ok(Box::new(ChannelTemplate {
            title: "Chat".to_owned(),
            group_id: 0,
            channel_id: 0,
            user_id: user.user_id,
            user_info: ser_json(&user_info),
            group_list: "[]".to_owned(),
            channel_list: "[]".to_owned(),
            url: "/channel/0/0".to_owned(),
        }))
    }

    let group_name = match group_list.iter().find(|g| g.group_id == group_id) {
        Some(group) => group.name.clone(),
        None => {
            group_id = group_list[0].group_id;
            channel_id = 0;
            group_list[0].name.clone()
        }
    };

    // This cannot be empty
    let channel_list = db::group_channels(pool.clone(), group_id).await?;

    let channel_name = match channel_list.iter().find(|c| c.channel_id == channel_id) {
        Some(channel) => channel.name.as_str(),
        None => {
            channel_id = channel_list[0].channel_id;
            channel_list[0].name.as_str()
        }
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
