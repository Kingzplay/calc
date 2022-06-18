pub fn split_keep<'a>(text: &String) -> Vec<String> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(|c: char| !(c == '.' ||c.is_alphanumeric() || c == '\'')) {
        if last != index {
            result.push(&text[last..index]);
        }
        result.push(matched);
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    let mut a = Vec::new();
    
    for i in result {
        a.push(i.to_string());
    }
a
}

