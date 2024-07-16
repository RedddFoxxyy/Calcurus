use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

mod operations;

struct Buttons {
    rect: Rect,
    text: String,
}
impl Buttons {
    fn dim(text: &str)-> Self {
        Buttons {
            rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            text: text.to_string(),
        }

    }

    fn update(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.rect = Rect::new(x, y, width, height);
    }

    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, GRAY);
        draw_text(&self.text, self.rect.x + 30.0, self.rect.y + 40.0, 30.0, BLACK);
    }

    fn clicked(&self) -> bool {
        let mouse_position = mouse_position();
        self.rect.contains(Vec2::from(mouse_position)) && is_mouse_button_pressed(MouseButton::Left)
    }
}

#[macroquad::main("Calcurus")]
async fn main() {
    set_window_size(400, 600);
    let mut border = screen_width() * 0.02;
    let mut border_h = screen_height() * 0.02;
    let mut usable_width = screen_width() - (4.0 * border);
    let mut usable_height = screen_height() - (7.0 * border);
    let mut width = usable_width / 3.0;
    let mut height = usable_height / 7.0;
    let mut col_1:f32 = border;
    let mut col_2:f32 = (border * 2.0) + width;
    let mut col_3:f32 = (border * 3.0) + (width * 2.0);
    let mut row_4:f32 = (border_h * 3.0) + (3.0 * height);
    let mut buttons4 = vec![
        Buttons::dim("1"),
        Buttons::dim("2"),
        Buttons::dim("3"),
    ];
    loop {
        clear_background(WHITE);
        border = (screen_width() / 100.0) * 2.0;
        border_h = (screen_height() / 100.0) * 2.0;
        usable_width = screen_width() - (4.0 * border);
        usable_height = screen_height() - (7.0 * border);
        width = usable_width / 3.0;
        height = usable_height / 7.0;
        col_1 = border;
        col_2 = (border * 2.0) + width;
        col_3 = (border * 3.0) + (width * 2.0);
        row_4 = (border_h * 3.0) + (3.0 * height);
        for (i, button) in buttons4.iter_mut().enumerate() {
            let x = border + (i as f32) * (width + border);
            button.update(x, row_4, width, height);
            button.draw();
        }
        if is_quit_requested() {
            break;
        }
        next_frame().await;
    }
}
