fn digital_root(n: i64) -> i64 {
    let mut num = n;
    while num > 9 {
        num = num.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>() as i64;
    }
    num
}
