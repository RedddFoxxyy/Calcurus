//! Handles the view logic of the App.

#[allow(unused_imports)]
use crate::{
	Calcurus, Message,
	calcurus::{
		defines::{NumObject, NumObjectBuffer},
		keys::*,
		logic::handle_key_click,
	},
};
use iced::{
	Element, Theme, alignment,
	widget::{button, center, column, row, text},
};

impl Calcurus {
	pub(crate) fn view(&self) -> Element<'_, Message> {
		// println!("Number of buttons in keyboard: {}", self.state.keyboard.len());
		let display: iced::widget::Text<Theme> = text(&self.display_buffer)
			.size(30)
			.width(iced::Length::Fill)
			.height(iced::Length::FillPortion(1))
			.align_x(alignment::Horizontal::Right);

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
				.color(iced::Color::new(0.9490196, 0.8980392, 0.7372549, 1.0))
				.size(25);

			let button = button(key_label)
				.on_press(Message::Click(key.clone()))
				.width(iced::Length::Fill)
				.height(iced::Length::Fill);

			let mut button_element: Element<Message> = button.into();
			if self.debug_mode {
				button_element = button_element.explain(iced::Color::WHITE);
			}

			current_row.push(button_element);

			// Create a new row after every 4 buttons
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
			.align_x(alignment::Horizontal::Center);

		let mut main_content: Element<Message> = center(content).into();
		if self.debug_mode {
			main_content = main_content.explain(iced::Color::WHITE);
		}
		main_content
	}
}
