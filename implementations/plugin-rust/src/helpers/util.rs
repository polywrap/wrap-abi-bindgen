pub fn replace_at(s: &str, idx: usize, replacement: &str) -> String {
    let mut chars = s.chars().collect::<Vec<_>>();
    for (i, r) in replacement.chars().enumerate() {
        chars[idx + i] = r;
    }
    chars.into_iter().collect()
}

pub fn insert_at(s: &str, idx: usize, insert: &str) -> String {
    let mut chars = s.chars().collect::<Vec<_>>();
    let insert_chars = insert.chars().collect::<Vec<_>>();
    for (i, char) in insert_chars.into_iter().enumerate() {
        chars.insert(idx + i, char);
    }
    chars.into_iter().collect()
}

pub fn remove_at(s: &str, idx: usize) -> String {
    let mut chars = s.chars().collect::<Vec<_>>();
    chars.remove(idx);
    chars.into_iter().collect()
}
