fn likes(names: &[&str]) -> String {
    let len = names.len();
    if len == 0 {
        return format!("no one likes this");
    } else if len == 1 {
        return format!("{} likes this", names[0]);
    } else if len == 2 {
        return format!("{} and {} like this", names[0], names[1]);
    } else if len == 3 {
        return format!("{}, {} and {} like this", names[0], names[1], names[2]);
    } else {
        return format!("{}, {} and {} others like this", names[0], names[1], len - 2);
    }
}
