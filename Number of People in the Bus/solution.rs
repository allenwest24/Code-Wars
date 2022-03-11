fn number(bus_stops:&[(i32,i32)]) -> i32 {
    let mut total = 0;
    for stop in bus_stops {
        total += stop.0 - stop.1;
    }
    return total;
}
