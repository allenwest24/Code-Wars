use std::collections::HashMap;

fn count_duplicates(text: &str) -> u32 {
    // Store counts in a HashMap.
    let mut char_counts : HashMap<char, i32> = HashMap::new();
    // Create a vector of chars in order.
    let char_vec : Vec<char> = text.to_lowercase().chars().collect();
    
    // Log the frequencies of each character seen.
    for c in char_vec {
        *char_counts.entry(c).or_insert(0) += 1;
    }
    
    // Use a counter and increment every time a key has been seen > 1 times.
    let mut x : u32 = 0;
    for (key, val) in char_counts.into_iter() {
        if val > 1 {
            x += 1;
        }
    }
    
    return x;
}
