pub fn string_replace(origin: &str, replace_by: &str, times: usize, content: String) -> String {
    return content.replacen(origin, replace_by, times)
}
