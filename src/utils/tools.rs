pub fn sort_strings_alphabetically(strings: &[String]) -> Vec<String> {
    let mut sorted_strings = strings.to_owned();
    sorted_strings.sort();
    sorted_strings
}
