fn valid_braces(s: &str) -> bool {
    let svec : Vec<char> = s.chars().collect();
    let mut left_vec = Vec::new();
    for ii in svec {
        if ii == '(' || ii == '[' || ii == '{' {
            left_vec.push(ii);
        } else {
            if left_vec.len() == 0 {return false;}
            let last = left_vec.last().copied().unwrap(); 
            if (ii == ')' && last != '(') || (ii == '}' && last != '{') || (ii == ']' && last != '[') {
                return false;
            }
            left_vec.pop();
        } 
    }
    if left_vec.len() == 0 {true} else {false}
