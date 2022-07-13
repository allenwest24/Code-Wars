fn accum(s:&str)->String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 {
            out.push('-');
        }
        out.push(c.to_ascii_uppercase());
        for j in 0..i {
            out.push(c.to_ascii_lowercase());
        }
    }
    out
}
