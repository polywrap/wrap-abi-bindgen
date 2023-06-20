pub fn replace_at(s: &str, idx: usize, replacement: &str) -> String {
    let start = s[..idx].to_string();
    let end = s[idx + replacement.len()..].to_string();
    format!("{}{}{}", start, replacement, end)
}

pub fn insert_at(s: &str, idx: usize, insert: &str) -> String {
    let start = s[..idx].to_string();
    let end = s[idx..].to_string();
    format!("{}{}{}", start, insert, end)
}

pub fn remove_at(s: &str, idx: usize) -> String {
    let start = s[..idx].to_string();
    let end = s[idx + 1..].to_string();
    format!("{}{}", start, end)
}

