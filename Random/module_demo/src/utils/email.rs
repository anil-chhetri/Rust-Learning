pub fn is_valid_email(address: &str) -> bool {
    let parts: Vec<_> = address.split("@").collect();
    if parts.len() != 2 {
        return false;
    }
    parts[1].contains(".")
}
