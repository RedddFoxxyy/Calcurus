use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use crate::operations::operate;

mod operations;

struct Buttons {
    rect: Rect,
    text: String,
    value: f64,
}
impl Buttons {
    fn dim(text: &str, value: f64)-> Self {
        Buttons {
            rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            text: text.to_string(),
            value,
        }

    }

    fn update(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.rect = Rect::new(x, y, width, height);
    }

    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, GRAY);
        let font_size = self.rect.h * 0.5;
        let text_dimensions = measure_text(&self.text, None, font_size as u16, 1.0);
        let text_x = self.rect.x + (self.rect.w - text_dimensions.width) * 0.5;
        let text_y = self.rect.y + (self.rect.h + text_dimensions.height) * 0.5;
        draw_text(&self.text, text_x, text_y, font_size, BLACK);
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
    let mut row_3:f32 = (border_h * 2.0) + (2.0 * height);
    let mut row_4:f32 = (border_h * 3.0) + (3.0 * height);
    let mut row_5:f32 = (border_h * 4.0) + (4.0 * height);
    let mut row_6:f32 = (border_h * 5.0) + (5.0 * height);
    let mut row_7:f32 = (border_h * 6.0) + (6.0 * height);
    let mut input_buffer = vec![];
    let mut clr:bool = false;
    let mut output:f64;
    let mut buttons3 = vec![
        Buttons::dim("+", 10.0),
        Buttons::dim("-", 11.0),
        Buttons::dim("*", 12.0),
    ];
    let mut buttons4 = vec![
        Buttons::dim("1", 1.0),
        Buttons::dim("2", 2.0),
        Buttons::dim("3", 3.0),
    ];
    let mut buttons5 = vec![
        Buttons::dim("4", 4.0),
        Buttons::dim("5", 5.0),
        Buttons::dim("6", 6.0),
    ];
    let mut buttons6 = vec![
        Buttons::dim("7", 7.0),
        Buttons::dim("8", 8.0),
        Buttons::dim("9", 9.0),
    ];
    let mut buttons7 = vec![
        Buttons::dim("0", 0.0),
        Buttons::dim("clr", 13.0),
        Buttons::dim("=", 14.0),
    ];
    loop {
        clear_background(WHITE);
        border = (screen_width() / 100.0) * 2.0;
        border_h = (screen_height() / 100.0) * 2.0;
        usable_width = screen_width() - (4.0 * border);
        usable_height = screen_height() - (7.0 * border_h);
        width = usable_width / 3.0;
        height = usable_height / 7.0;
        row_3 = (border_h * 2.0) + (2.0 * height);
        row_4 = (border_h * 3.0) + (3.0 * height);
        row_5 = (border_h * 4.0) + (4.0 * height);
        row_6 = (border_h * 5.0) + (5.0 * height);
        row_7 = (border_h * 6.0) + (6.0 * height);
        for (i, button) in buttons3.iter_mut().enumerate() {
            let x = border + (i as f32) * (width + border);
            button.update(x, row_3, width, height);
            button.draw();
            if button.clicked() {
                println!("Button {} clicked!", button.text);
                input_buffer.push(button.value);
            }
        }
        for (i, button) in buttons4.iter_mut().enumerate() {
            let x = border + (i as f32) * (width + border);
            button.update(x, row_4, width, height);
            button.draw();
            if button.clicked() {
                println!("Button {} clicked!", button.text);
                input_buffer.push(button.value);
            }
        }
        for (i, button) in buttons5.iter_mut().enumerate() {
            let x = border + (i as f32) * (width + border);
            button.update(x, row_5, width, height);
            button.draw();
            if button.clicked() {
                println!("Button {} clicked!", button.text);
                input_buffer.push(button.value);
            }
        }
        for (i, button) in buttons6.iter_mut().enumerate() {
            let x = border + (i as f32) * (width + border);
            button.update(x, row_6, width, height);
            button.draw();
            if button.clicked() {
                println!("Button {} clicked!", button.text);
                input_buffer.push(button.value);
            }
        }
        for (i, button) in buttons7.iter_mut().enumerate() {
            let x = border + (i as f32) * (width + border);
            button.update(x, row_7, width, height);
            button.draw();
            if button.clicked() {
                println!("Button {} clicked!", button.text);
                if button.value == 0.0 {
                    input_buffer.push(button.value);
                }
                if button.value == 13.0 {
                    clr = true;
                }
                if button.value== 14.0 {
                    output = operations::operate(&input_buffer).await;
                    println!("{}", output);
                }
            }
        }
        if clr == true {
            input_buffer.clear();
            println!("Input Buffer Cleared");
            clr = false;
        }
        if is_quit_requested() {
            break;
        }
        next_frame().await;
    }
}
