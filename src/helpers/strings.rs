use std::iter::FromIterator;

pub fn into_clean_string(s: &String) -> String {
    String::from_iter(s.chars().filter(|&c| c != '\u{0}').collect::<Vec<char>>())
}