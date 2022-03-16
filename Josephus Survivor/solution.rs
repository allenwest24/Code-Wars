fn josephus_survivor(n: i32, k: i32) -> i32 {
    let mut vec : Vec<i32> = (1..n+1).collect();
    let mut pos = 0;
    while vec.len() > 1 {
        pos = (pos + (k - 1)).rem_euclid(vec.len() as i32);
        vec.remove(pos as usize);
    }
    vec[0]
}
