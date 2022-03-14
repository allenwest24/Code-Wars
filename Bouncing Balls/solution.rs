fn bouncing_ball(mut h: f64,  bounce: f64,  window: f64) -> i32 {
    let mut x: i64 = -1;
    if h <= 0.0 || bounce <= 0.0 || bounce >= 1.0 || window >= h {
        return x as i32;
    }
    
    while h > window {
        x += 2;
        h *= bounce;
    }
    x as i32
}
