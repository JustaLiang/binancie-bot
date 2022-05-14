pub fn reply(string: String) -> String {
    string.chars().map(|c| c.to_ascii_lowercase()).collect()
}