fn persistence(num: u64) -> u64 {
    // Counter for amount of iterations needed.
    let mut x = 0;
    // Borrowing.
    let mut curr = num;
    
    // Go until single digit.
    while curr >= 10 {
        // Break into a vector of digits in this number.
        let digits: Vec<_> = curr.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
        // Start new curr at a number it can be multiplied by.
        curr = 1;
        // Multiply all digits together.
        for d in digits {
            curr *= d as u64;
        }
        // Increment counter.
        x += 1;
    }
    
    return x;
}
