use std::collections::BTreeMap;

fn prime_factors(n: i64) -> String {
    // Using a BTreeMap which is a HashMap sorted by Keys.
    let mut factors: BTreeMap<i64, i32> = BTreeMap::new();
    let mut curr = n;
    let mut done = false;
    
    // Keep finding the smallest prime factor.
    loop {
        for ii in 2..curr + 1 {
            // If the smallest factor is the number itself, we are done after we add.
            if ii == curr {
                done = true;
            }
            // Add the factor to our current BTreeMap.
            if curr % ii == 0 {
                *factors.entry(ii).or_insert(0) += 1;
                curr = curr / ii;
                break;
            }
        }
        if done {break;}
    }
    
    // Build the output string.
    let mut out = String::new();
    for (key, val) in factors {
        if val > 1 {
            out += &*format!("({}**{})", key, val);
        } else {
            out += &*format!("({})", key);
        }   
    }
    
    return out;
}
