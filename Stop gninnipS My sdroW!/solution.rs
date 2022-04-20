fn spin_words(words: &str) -> String {
    let mut split = words.split(" ");
    let mut out = String::new();
    for ii in split {
        if out.len() > 0 {out.push_str(" ")}
        if ii.len() >= 5 {
            out.push_str(&ii.chars().rev().collect::<String>());
        } else {
            out.push_str(ii);
        }
    }
    out
}
