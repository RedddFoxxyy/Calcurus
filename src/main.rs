use macroquad::prelude::*;
mod operations;

#[macroquad::main("My Calculator")]
async fn main() {
    loop {
        clear_background(WHITE);
        if is_quit_requested() {
            break;
        }
        next_frame().await;
    }
}
