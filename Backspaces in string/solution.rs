fn clean_string(s: &str) -> String {
    let mut out : Vec<char> = Vec::new();
    for c in s.chars() {
        if c == '#' && out.len() != 0 {
            out.pop();
        } else if c != '#' {
            out.push(c)
        }
    }
    out.into_iter().collect()
}
