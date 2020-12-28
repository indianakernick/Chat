use rand::Rng;

/// Generate a [base64url][1] encoded, cryptographically secure, random string.
///
/// [1]: https://tools.ietf.org/html/rfc4648#page-7
pub fn generate_random_base64url(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0; length];

    for i in 0..length {
        let num: u8 = rng.gen_range(0, 64);
        if num < 26 {
            bytes[i] = b'A' + num;
        } else if num < 2 * 26 {
            bytes[i] = b'a' + num - 26;
        } else if num < 2 * 26 + 10 {
            bytes[i] = b'0' + num - 2 * 26;
        } else if num == 2 * 26 + 10 {
            bytes[i] = b'-';
        } else {
            bytes[i] = b'_';
        }
    }

    unsafe {
        return String::from_utf8_unchecked(bytes);
    }
}
