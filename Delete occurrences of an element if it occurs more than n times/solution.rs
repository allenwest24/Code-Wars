fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    // Create a new list.
    let mut out : Vec<u8> = vec![];
    
    // Go through the list and check if we can store more of a given item.
    for ii in lst {
        if out.iter().filter(|&x| *x == *ii).count() < n {
            out.push(*ii);
        }
    }
    
    return out;
}
