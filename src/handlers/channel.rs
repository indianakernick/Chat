use askama::Template;
use serde::Serialize;
use crate::database as db;
use deadpool_postgres::Pool;

#[derive(Template)]
#[template(path = "../client/dist/channel.html")]
struct ChannelTemplate {
    title: String,
    user_info: String,
    group_info: String,
    channel_list: String,
    channel_id: db::ChannelID,
    group_id: db::GroupID,
    user_id: db::UserID,
}

fn ser_json<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value).unwrap().replace("</script>", "<\\/script>")
}

pub async fn channel(group_id: db::GroupID, channel_id: db::ChannelID, session_id: db::SessionID, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let (group, channel_list, session) = futures::future::join3(
        db::group_info(pool.clone(), group_id),
        db::group_channels(pool.clone(), group_id),
        db::session_info(pool.clone(), session_id)
    ).await;

    let group = match group? {
        Some(info) => info,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };

    let channel_list = channel_list?;

    let mut channel_name = None;
    for info in &channel_list {
        if info.channel_id == channel_id {
            channel_name = Some(info.name.clone());
            break;
        }
    }

    let channel_name = match channel_name {
        Some(name) => name,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };

    let session = match session? {
        Some(s) => s,
        None => return Ok(Box::new(warp::redirect(
            format!("/login?redirect=/channel/{}/{}", group_id, channel_id)
                .parse::<warp::http::Uri>().unwrap()
        )))
    };

    let user_info = db::UserInfo {
        name: session.name,
        picture: session.picture
    };

    Ok(Box::new(ChannelTemplate {
        title: group.name.clone() + "#" + channel_name.as_str(),
        user_info: ser_json(&user_info),
        group_info: ser_json(&group),
        channel_list: ser_json(&channel_list),
        channel_id,
        group_id,
        user_id: session.user_id
    }))
}
