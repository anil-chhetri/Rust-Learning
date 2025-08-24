pub(crate) fn normalise_phone(input: &str) -> String {
    input.chars().filter(|x| x.is_ascii_digit()).collect()
}
