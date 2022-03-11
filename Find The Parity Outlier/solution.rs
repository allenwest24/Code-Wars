fn find_outlier(values: &[i32]) -> i32 {
    return if (values.into_iter().filter(|&x| x.rem_euclid(2) == 1).count() == 1) {*values.into_iter().filter(|&x| x.rem_euclid(2) == 1).collect::<Vec<&i32>>()[0]} else {*values.into_iter().filter(|&x| x.rem_euclid(2) == 0).collect::<Vec<&i32>>()[0]};
}
