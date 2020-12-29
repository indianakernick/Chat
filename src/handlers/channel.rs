use askama::Template;
use serde::Serialize;
use crate::database as db;
use deadpool_postgres::Pool;

#[derive(Template)]
#[template(path = "channel.html")]
struct ChannelTemplate {
    title: String,
    preload_images: Vec<String>,
    group_id: db::GroupID,
    channel_id: db::ChannelID,
    user_id: db::UserID,
    user_list: String,
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

    let group_list = db::user_groups(pool.clone(), user.user_id).await?;

    if group_list.is_empty() {
        let preload_images = vec![user.picture.clone()];
        let user_id = user.user_id;
        let user_list = vec![user];
        return Ok(Box::new(ChannelTemplate {
            title: "Chat".to_owned(),
            preload_images,
            group_id: 0,
            channel_id: 0,
            user_id,
            user_list: ser_json(&user_list),
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

    let (channel_list, user_list) = futures::future::join(
        db::group_channels(pool.clone(), group_id),
        db::group_users(pool.clone(), group_id)
    ).await;

    // The channel list cannot be empty
    let channel_list = channel_list?;
    let user_list = user_list?;

    let channel_name = match channel_list.iter().find(|c| c.channel_id == channel_id) {
        Some(channel) => channel.name.as_str(),
        None => {
            channel_id = channel_list[0].channel_id;
            channel_list[0].name.as_str()
        }
    };

    let mut preload_images = Vec::new();
    for group in group_list.iter() {
        preload_images.push(group.picture.clone());
    }
    for other_user in user_list.iter() {
        preload_images.push(other_user.picture.clone());
    }

    Ok(Box::new(ChannelTemplate {
        title: group_name + "#" + channel_name,
        preload_images,
        group_id,
        channel_id,
        user_id: user.user_id,
        user_list: ser_json(&user_list),
        group_list: ser_json(&group_list),
        channel_list: ser_json(&channel_list),
        url: format!("/channel/{}/{}", group_id, channel_id),
    }))
}
