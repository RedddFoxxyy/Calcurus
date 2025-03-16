#[allow(unused_imports)]
use iced::{
    Element, Size, Theme, alignment,
    widget::{button, column, row, text},
    window,
};
use rust_decimal::Decimal;
// use rust_decimal::*;
// use rust_decimal_macros::dec;

use crate::calcurus::{keys::*, operations::operate};

// #[derive(Clone)]
// pub struct Operator {
//     pub value: String,
// }

#[derive(Clone)]
pub enum NumObject {
    DecNumber(Decimal),
    Operator(String),
}

pub struct NumObjectBuffer {
    pub len: usize,
    pub buffer: Vec<NumObject>,
    pub current_object: Option<NumObject>,
}
impl NumObjectBuffer {
    pub fn default() -> Self {
        NumObjectBuffer {
            len: 0,
            buffer: Vec::new(),
            current_object: None,
        }
    }

    pub fn push(&mut self, num_object: NumObject) {
        self.buffer.push(num_object);
        self.len += 1;
    }

    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<NumObject> {
        if self.len > 0 {
            self.len -= 1;
            self.buffer.pop()
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.len = 0;
        self.buffer.clear();
    }
}

// pub type NumDisplayBuffer = Vec<String>;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Click(String),
}

pub(crate) struct Calcurus {
    pub num_buffer: NumObjectBuffer,
    pub display_buffer: String,
    pub num_string_buffer: String,
    pub is_output_dec: bool,
    pub keyboard: Vec<String>,
}

impl Default for Calcurus {
    fn default() -> Self {
        let keys: Vec<String> = generate_key_layout();

        Self {
            num_buffer: NumObjectBuffer::default(),
            display_buffer: String::new(),
            // thought [initialisation]: Should this be initialised as true or not?
            is_output_dec: true,
            num_string_buffer: String::new(),
            keyboard: keys,
        }
    }
}

impl Calcurus {
    pub(crate) fn update(&mut self, message: Message) {
        match message {
            Message::Click(button_id) => {
                if button_id.as_str() == "Clr" {
                    self.num_buffer.clear();
                    self.display_buffer.clear();
                    self.num_string_buffer.clear();
                    self.is_output_dec = true;
                } else if button_id.as_str() == "Bck" {
                    if self.is_output_dec {
                        if self.num_string_buffer.is_empty() {
                            self.num_buffer.pop();
                            self.display_buffer.pop();
                        } else {
                            self.display_buffer.pop();
                            self.num_string_buffer.pop();
                        }
                    } else {
                        self.num_buffer.clear();
                        self.display_buffer.clear();
                        self.num_string_buffer.clear();
                        self.is_output_dec = true;
                    }
                }
                //
                // TODO: Replace unwrap with unwrap_or_else.
                let button_id_char = button_id.chars().next().unwrap();
                match button_id_char {
                    '0'..='9' | '.' => {
                        if !self.is_output_dec {
                            self.num_buffer.clear();
                            self.display_buffer.clear();
                            self.num_string_buffer.clear();
                            self.num_string_buffer.push(button_id_char);
                            self.display_buffer.push(button_id_char);
                            self.is_output_dec = true;
                        } else {
                            self.num_string_buffer.push(button_id_char);
                            self.display_buffer.push(button_id_char);
                        }
                    }
                    // TODO: Add handling case for '√'
                    '+' | '-' | '*' | '/' | '^' => {
                        if self.num_string_buffer.is_empty() {
                            if button_id_char == '+' || button_id_char == '-' {
                                self.num_string_buffer.push(button_id_char);
                                self.display_buffer.push(button_id_char);
                            }
                            return;
                        }
                        let new_num = self.num_string_buffer.parse::<Decimal>().unwrap();
                        self.num_string_buffer.clear();
                        if self.num_buffer.current_object.is_none() {
                            self.num_buffer.current_object = Some(NumObject::DecNumber(new_num));
                            let operator = NumObject::Operator(button_id);
                            self.num_buffer.push(operator);
                            self.display_buffer.push(button_id_char);
                        } else {
                            let current_num_object =
                                self.num_buffer.current_object.clone().unwrap();
                            self.num_buffer.current_object = None;
                            let operator = NumObject::Operator(button_id);
                            self.num_buffer.push(current_num_object);
                            self.num_buffer.push(operator);
                            self.display_buffer.push(button_id_char);
                        }
                    }
                    '=' => {
                        // Parse and add the current number to num_buffer before operating
                        if !self.num_string_buffer.is_empty() {
                            let final_num = self.num_string_buffer.parse::<Decimal>().unwrap();
                            self.num_string_buffer.clear();

                            let num_object = NumObject::DecNumber(final_num);
                            if self.num_buffer.current_object.is_some() {
                                let current_obj = self.num_buffer.current_object.take().unwrap();
                                self.num_buffer.push(current_obj);
                            }
                            self.num_buffer.push(num_object);
                        }
                        operate(self);
                        if self.num_buffer.buffer.len() == 1 {
                            let current_num_object: NumObject = self.num_buffer.buffer[0].clone();

                            if let NumObject::DecNumber(current_num) = current_num_object {
                                let current_num_string = current_num.to_string();
                                self.num_string_buffer.push_str(&current_num_string);
                            }

                            self.num_buffer.clear();
                        }
                    }

                    _ => (), // Handle all other cases
                }
            }
        }
    }

    pub(crate) fn view(&self) -> Element<Message> {
        // println!("Number of buttons in keyboard: {}", self.state.keyboard.len());
        let display: iced::widget::Text<iced::Theme> = text(&self.display_buffer)
            .size(40)
            .width(iced::Length::Fill)
            .height(iced::Length::FillPortion(1))
            .align_x(iced::alignment::Horizontal::Right);

        // Create a grid of buttons from the keyboard
        let mut button_rows: Vec<Element<Message>> = Vec::new();
        let mut current_row: Vec<Element<Message>> = Vec::new();

        // Iterate through all buttons in the keyboard
        for (index, key) in self.keyboard.iter().enumerate() {
            let key_label = text(key)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
                .size(25);
            let button_element = button(key_label)
                .on_press(Message::Click(key.clone()))
                .width(iced::Length::Fill)
                .height(iced::Length::Fill);

            current_row.push(button_element.clip(false).into());

            // Create a new row after every 3 buttons
            if current_row.len() == 4 || index == self.keyboard.len() - 1 {
                button_rows.push(row(std::mem::take(&mut current_row)).spacing(3).into());
            }
        }

        let keys_column: iced::widget::Column<_> = column(button_rows)
            .spacing(3)
            .height(iced::Length::FillPortion(4));
        let content: iced::widget::Column<_> = column![display, keys_column]
            .padding(5)
            .spacing(5)
            .width(iced::Length::Fill)
            .align_x(iced::alignment::Horizontal::Center);
        iced::widget::center(content).into()
    }
}
