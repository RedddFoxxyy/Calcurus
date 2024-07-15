use macroquad::prelude::*;

async fn add(vec: &Vec<i64>)->i64 {
    let mut sum: i64 = 0;
    for num in vec {
        sum += num;
    }
    return sum;
}
