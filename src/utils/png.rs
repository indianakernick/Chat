pub fn optimize_png(input_data: &[u8]) -> Option<Vec<u8>> {
    let mut options = oxipng::Options::from_preset(3);
    options.fix_errors = true;
    options.force = true;
    options.interlace = Some(0);
    options.strip = oxipng::Headers::All;
    options.timeout = Some(std::time::Duration::from_millis(100));
    match oxipng::optimize_from_memory(input_data, &options) {
        Ok(picture) => Some(picture),
        Err(_) => None
    }
}
