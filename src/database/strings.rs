pub const MAX_CHANNEL_NAME_LENGTH: usize = 32;
pub const MAX_GROUP_NAME_LENGTH: usize = 32;
pub const MAX_URL_LENGTH: usize = 2048;
pub const MAX_USER_NAME_LENGTH: usize = 64;
pub const MAX_MESSAGE_LENGTH: usize = 1024;

pub fn valid_channel_name(name: &String) -> bool {
    // A byte limit instead of a character limit is tempting...
    if name.is_empty() || name.len() > 4 * MAX_CHANNEL_NAME_LENGTH {
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

fn within_char_limit(string: &String, max_chars: usize) -> bool {
    string.len() <= 4 * max_chars && string.chars().count() <= max_chars
}

pub fn valid_group_name(name: &String) -> bool {
    !name.is_empty() && within_char_limit(name, MAX_GROUP_NAME_LENGTH)
}

pub fn valid_url(url: &String) -> bool {
    within_char_limit(url, MAX_URL_LENGTH) && reqwest::Url::parse(url).is_ok()
}

// TODO: Enforce this on user creation somehow. Or don't...
pub fn valid_user_name(name: &String) -> bool {
    !name.is_empty() && within_char_limit(name, MAX_USER_NAME_LENGTH)
}

pub fn valid_message(message: &String) -> bool {
    !message.is_empty() && within_char_limit(message, MAX_MESSAGE_LENGTH)
}
