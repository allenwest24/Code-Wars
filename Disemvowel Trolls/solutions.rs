fn disemvowel(s: &str) -> String {
    return (s
        .replace("a", "")
        .replace("e", "")
        .replace("i", "")
        .replace("o", "")
        .replace("u", "")
        .replace("A", "")
        .replace("E", "")
        .replace("I", "")
        .replace("O", "")
        .replace("U", ""));
}
