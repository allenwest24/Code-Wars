fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;
    for c in string.chars() {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            vowels_count += 1;
        }
    }
    vowels_count
}
