use macroquad::prelude::*;

async fn operate(vec: &Vec<f64>, value: f64)->f64 {
    let mut sum: f64 = 0.0;
    for num in vec {
        sum += num;
    }
    return sum;
}
