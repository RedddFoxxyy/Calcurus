#![windows_subsystem = "windows"]

use macroquad::prelude::*;

use crate::operations::{concatenate_strings, key_check};

mod operations;

pub const BASE:Color = Color::new(0.1176470588235294, 0.1176470588235294, 0.1803921568627451, 1.0);
pub const SURFACE1:Color = Color::new(0.2705882352941176, 0.2784313725490196, 0.3529411764705882, 1.0);
pub const SURFACE1HOVERED:Color = Color::new(0.49803921568627450980392156862745, 0.51764705882352941176470588235294, 0.61176470588235294117647058823529, 1.0);
pub const TEXT:Color = Color::new(0.803921568627451, 0.8392156862745098, 0.9568627450980392, 1.0);

struct ScreenRect {
    rect: Rect,
    text: String,
}
impl ScreenRect {
    fn dim(text: &str) -> Self {
        ScreenRect {
            rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            text: text.to_string(),
        }
    }

    fn update(&mut self, x: f32, y: f32, width: f32, height: f32, input: String) {
        self.rect = Rect::new(x, y, width, height);
        self.text = input;
    }

    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BASE);
        let mut font_size = self.rect.h * 0.4;
        let mut text_dimensions = measure_text(&self.text, None, font_size as u16, 1.0);

        if text_dimensions.width > self.rect.w {
            font_size = (self.rect.w * font_size) / text_dimensions.width;
            text_dimensions = measure_text(&self.text, None, font_size as u16, 1.0);
        }

        let text_x = self.rect.x + self.rect.w - text_dimensions.width + 10.0;
        let text_y = self.rect.y + (self.rect.h + text_dimensions.height) * 0.25;
        draw_text(&self.text, text_x, text_y, font_size, TEXT);
    }
}

struct Buttons {
    rect: Rect,
    text: String,
    value: u8,
}
impl Buttons {
    fn dim(text: &str, value: u8) -> Self {
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
        let color = if self.rect.contains(Vec2::from(mouse_position())) {
            SURFACE1HOVERED
        } else {
            SURFACE1
        };
        
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
        let font_size = self.rect.h * 0.5;
        let text_dimensions = measure_text(&self.text, None, font_size as u16, 1.0);
        let text_x = self.rect.x + (self.rect.w - text_dimensions.width) * 0.5;
        let text_y = self.rect.y + (self.rect.h + text_dimensions.height) * 0.5;
        draw_text(&self.text, text_x, text_y, font_size, TEXT);
    }

    fn clicked(&self) -> bool {
        self.rect.contains(Vec2::from(mouse_position())) && is_mouse_button_pressed(MouseButton::Left)
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Calcurus".to_owned(),
        window_width: 400,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    clear_background(BASE);
    let mut border = screen_width() * 0.02;
    let mut border_h = screen_height() * 0.02;
    let mut usable_width = screen_width() - (5.0 * border);
    let mut usable_height = screen_height() - (7.0 * border);
    let mut width = usable_width / 4.0;
    let mut height = usable_height / 7.0;
    let mut row_3: f32 = (border_h * 2.0) + (2.0 * height);
    let mut row_4: f32 = (border_h * 3.0) + (3.0 * height);
    let mut row_5: f32 = (border_h * 4.0) + (4.0 * height);
    let mut row_6: f32 = (border_h * 5.0) + (5.0 * height);
    let mut row_7: f32 = (border_h * 6.0) + (6.0 * height);
    let mut input_buffer = vec![];
    let mut clr: bool = false;
    let mut output: f64;
    let mut display = ScreenRect::dim("input");
    let mut display_height = height * 2.0;
    display.update(border, border_h, usable_width, display_height, "input".to_string());
    let mut display_buffer = vec![];
    let mut decimal_present = false;
    let divide_by_zero = "Can't divide by 0.";
    let mut buttons3 = vec![
        Buttons::dim("+", 10),
        Buttons::dim("-", 11),
        Buttons::dim("×", 12),
        Buttons::dim("÷", 15),
    ];
    let mut buttons4 = vec![
        Buttons::dim("1", 1),
        Buttons::dim("2", 2),
        Buttons::dim("3", 3),
        //Buttons::dim("√", 17.0),
    ];
    let mut buttons5 = vec![
        Buttons::dim("4", 4),
        Buttons::dim("5", 5),
        Buttons::dim("6", 6),
    ];
    let mut buttons6 = vec![
        Buttons::dim("7", 7),
        Buttons::dim("8", 8),
        Buttons::dim("9", 9),
        Buttons::dim("bck", 18),
    ];
    let mut buttons7 = vec![
        Buttons::dim("0", 0),
        Buttons::dim("clr", 13),
        Buttons::dim(".", 16),
        Buttons::dim("=", 14),
    ];
    loop {
        clear_background(BASE);
        border = (screen_width() / 100.0) * 2.0;
        border_h = (screen_height() / 100.0) * 2.0;
        usable_width = screen_width() - (5.0 * border);
        usable_height = screen_height() - (7.0 * border_h);
        width = usable_width / 4.0;
        height = usable_height / 7.0;
        display_height = height * 2.0;
        row_3 = (border_h * 2.0) + (2.0 * height);
        row_4 = (border_h * 3.0) + (3.0 * height);
        row_5 = (border_h * 4.0) + (4.0 * height);
        row_6 = (border_h * 5.0) + (5.0 * height);
        row_7 = (border_h * 6.0) + (6.0 * height);
        display.draw();
        for (i, button) in buttons3.iter_mut().enumerate() {
            let x = border + (i as f32) * (width + border);
            button.update(x, row_3, width, height);
            button.draw();
            if button.clicked() || key_check(button.value).await {
                if input_buffer.is_empty() && (button.value == 10 || button.value == 11) {
                    input_buffer.push(button.value);
                    display_buffer.push(button.text.to_string());
                } else if !input_buffer.is_empty() {
                    if input_buffer.last().unwrap() != &10
                        && input_buffer.last().unwrap() != &11
                        && input_buffer.last().unwrap() != &12
                        && input_buffer.last().unwrap() != &15
                    {
                        input_buffer.push(button.value);
                        display_buffer.push(button.text.to_string());
                        decimal_present = false;
                    }
                }
                //println!("Button {} clicked!", button.text);
            }
        }
        for (i, button) in buttons4.iter_mut().enumerate() {
            let x = border + (i as f32) * (width + border);
            button.update(x, row_4, width, height);
            button.draw();
            if button.clicked() || key_check(button.value).await {
                //println!("Button {} clicked!", button.text);
                input_buffer.push(button.value);
                display_buffer.push(button.text.to_string());
            }
        }
        for (i, button) in buttons5.iter_mut().enumerate() {
            let x = border + (i as f32) * (width + border);
            button.update(x, row_5, width, height);
            button.draw();
            if button.clicked() || key_check(button.value).await {
                //println!("Button {} clicked!", button.text);
                input_buffer.push(button.value);
                display_buffer.push(button.text.to_string());
            }
        }
        for (i, button) in buttons6.iter_mut().enumerate() {
            let x = border + (i as f32) * (width + border);
            button.update(x, row_6, width, height);
            button.draw();
            if button.clicked() || key_check(button.value).await {
                if button.value != 18 {
                    //println!("Button {} clicked!", button.text);
                    input_buffer.push(button.value);
                    display_buffer.push(button.text.to_string());
                } else {
                    if !input_buffer.is_empty() {
                        if input_buffer.last().unwrap() == &16 {
                            decimal_present = false;
                        }
                        input_buffer.remove(input_buffer.len() - 1);
                        display_buffer.remove(display_buffer.len() - 1);
                    }
                }
            }
        }
        for (i, button) in buttons7.iter_mut().enumerate() {
            let x = border + (i as f32) * (width + border);
            button.update(x, row_7, width, height);
            button.draw();
            if button.clicked() || key_check(button.value).await {
                //println!("Button {} clicked!", button.text);
                if button.value == 0 {
                    if !input_buffer.is_empty() && input_buffer.last().unwrap() != &15 {
                        input_buffer.push(button.value);
                        display_buffer.push(button.text.to_string());
                    } else if input_buffer.is_empty() {
                        input_buffer.push(button.value);
                        display_buffer.push(button.text.to_string());
                    } else {
                        input_buffer.clear();
                        display_buffer.clear();
                        display_buffer.push(divide_by_zero.to_string());
                    }
                }
                if button.value == 16 && !decimal_present {
                    if !input_buffer.is_empty() {
                        if input_buffer.last().unwrap() != &16 {
                            input_buffer.push(button.value);
                            display_buffer.push(button.text.to_string());
                        }
                    } else {
                        input_buffer.push(button.value);
                        display_buffer.push(button.text.to_string());
                    }
                    decimal_present = true;
                }
                if button.value == 13 {
                    clr = true;
                }
                if button.value == 14 {
                    output = operations::operate(&input_buffer).await;
                    //println!("{}", output);
                    decimal_present = false;
                    input_buffer.clear();
                    operations::f64_to_u8_vec(output, &mut input_buffer, &mut decimal_present);
                    display_buffer.clear();
                    display_buffer.push(format!("{}", output));
                }
            }
        }
        if clr == true {
            input_buffer.clear();
            display_buffer.clear();
            //println!("Input Buffer Cleared");
            clr = false;
            decimal_present = false;
        }

        display.update(border, border_h, usable_width, display_height, concatenate_strings(&display_buffer).await);
        display.draw();

        if is_quit_requested() {
            break;
        }
        next_frame().await;
    }
}
