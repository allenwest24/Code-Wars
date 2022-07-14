fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut vec = Vec::new();
    for ii in a {
        if !b.contains(&ii) {
            vec.push(ii);
        }
    }
    vec
}
