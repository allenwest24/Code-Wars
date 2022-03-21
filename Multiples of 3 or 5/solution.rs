fn solution(num: i32) -> i32 {
    if num < 0 {return 0;}
    let mut x = 0;
    for ii in 0..num {
        if ii.rem_euclid(5) == 0 || ii.rem_euclid(3) == 0 {x += ii} 
    }
    x
}
