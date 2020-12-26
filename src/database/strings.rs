pub const MAX_CHANNEL_NAME_LENGTH: usize = 32;
pub const MAX_GROUP_NAME_LENGTH: usize = 32;
pub const MAX_URL_LENGTH: usize = 2048;

pub fn valid_channel_name(name: &String) -> bool {
    // A byte limit instead of a character limit is tempting...
    if name.is_empty() {
        return false;
    }
    if name.len() > 4 * MAX_CHANNEL_NAME_LENGTH {
        return false;
    }

    let mut count = 0;

    for ch in name.chars() {
        count += 1;
        if count > MAX_CHANNEL_NAME_LENGTH {
            return false;
        }
        if ch == '#' || ch == '@' || ch.is_whitespace() {
            return false;
        }
    }

    return true;
}

pub fn valid_group_name(name: &String) -> bool {
    if name.is_empty() {
        return false;
    }
    if name.len() > 4 * MAX_GROUP_NAME_LENGTH {
        return false;
    }
    return name.chars().count() <= MAX_GROUP_NAME_LENGTH;
}

// TODO: valid_user_name

pub fn valid_url(url: &String) -> bool {
    return url.len() < 4 * MAX_URL_LENGTH && url.chars().count() <= MAX_URL_LENGTH;
}
