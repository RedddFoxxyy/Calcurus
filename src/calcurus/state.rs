#[allow(unused_imports)]
use iced::{
    Element, Size, Theme, alignment,
    widget::{button, column, row, text},
    window,
};
use rust_decimal::Decimal;
// use rust_decimal::*;
// use rust_decimal_macros::dec;

use crate::calcurus::{keys::*, operations::operate_on};

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
    pub keyboard: Vec<String>,
}

impl Default for Calcurus {
    fn default() -> Self {
        let keys: Vec<String> = generate_key_layout();

        Self {
            num_buffer: NumObjectBuffer::default(),
            display_buffer: String::new(),
            num_string_buffer: String::new(),
            keyboard: keys,
        }
    }
}

// if self.num_buffer.current_object.is_none() {
//     self.num_buffer.current_object = Some(NumObject::DecNumber(dec!(num)));
// }
impl Calcurus {
    pub(crate) fn update(&mut self, message: Message) {
        match message {
            Message::Click(button_id) => {
                if button_id.as_str() == "Clr" {
                    self.num_buffer.clear();
                    self.display_buffer.clear();
                    self.num_string_buffer.clear();
                }
                // else if button_id.as_str() == "Bck" {
                //     self.num_buffer.pop();
                //     self.display_buffer.pop();
                //     self.num_string_buffer.clear();
                // }
                // TODO: Replace unwrap with unwrap_or_else.
                let button_id_char = button_id.chars().next().unwrap();
                match button_id_char {
                    '0'..='9' | '.' => {
                        self.num_string_buffer.push(button_id_char);
                        self.display_buffer.push(button_id_char);
                    }
                    '+' | '-' | '*' | '/' => {
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
                            let new_num = self.num_string_buffer.parse::<Decimal>().unwrap();
                            self.num_string_buffer.clear();

                            let num_object = NumObject::DecNumber(new_num);
                            if self.num_buffer.current_object.is_some() {
                                let current_obj = self.num_buffer.current_object.take().unwrap();
                                self.num_buffer.push(current_obj);
                            }
                            self.num_buffer.push(num_object);
                        }
                        operate_on(&self.num_buffer, &mut self.display_buffer)
                    }

                    _ => (), // Handle all other cases
                }
            }
        }
    }

    pub(crate) fn view(&self) -> Element<Message> {
        // println!("Number of buttons in keyboard: {}", self.state.keyboard.len());
        let display: iced::widget::Text<iced::Theme> = text(&self.display_buffer)
            .size(50)
            .width(iced::Length::Fill)
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
                .size(20);
            let button_element = button(key_label)
                .on_press(Message::Click(key.clone()))
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .padding(2);

            current_row.push(button_element.into());

            // Create a new row after every 3 buttons
            if current_row.len() == 3 || index == self.keyboard.len() - 1 {
                button_rows.push(row(std::mem::take(&mut current_row)).spacing(10).into());
            }
        }

        let keys_column: iced::widget::Column<_> = column(button_rows).spacing(10);
        let content: iced::widget::Column<_> = column![display, keys_column]
            .padding(10)
            .spacing(20)
            .width(iced::Length::Fill)
            .align_x(iced::alignment::Horizontal::Center);
        iced::widget::center(content).into()
    }
}
