#[allow(unused_imports)]
use crate::calcurus::{
	defines::{NumObject, NumObjectBuffer},
	keys::*,
	logic::handle_key_click,
};
use iced::{
	Element, Theme, alignment,
	widget::{button, center, column, row, text},
};

#[derive(Debug, Clone)]
pub(crate) enum Message {
	Click(String),
}

pub(crate) struct Calcurus {
	/// Buffer to store NumObjects to Operate on.
	pub num_buffer: NumObjectBuffer,
	/// Buffer to store the string that will be displayed on the calculator.
	pub display_buffer: String,
	/// Temporary buffer to store the number that is being typed.
	pub current_input_buffer: String,
	/// Used to track if the output of last operation was decimal value or not.
	pub is_output_dec: bool,
	/// Stores the Keyboard Keys.
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
			current_input_buffer: String::new(),
			keyboard: keys,
		}
	}
}

impl Calcurus {
	/// Push a character to the Num String Buffer and update it on the Display Buffer.
	pub(crate) fn push_current_input(&mut self, char: &char) {
		self.current_input_buffer.push(*char);
		self.display_buffer.push(*char);
	}

	pub(crate) fn update(&mut self, message: Message) {
		match message {
			Message::Click(button_id) => handle_key_click(self, button_id),
		}
	}

	pub(crate) fn view(&self) -> Element<Message> {
		// println!("Number of buttons in keyboard: {}", self.state.keyboard.len());
		let display: iced::widget::Text<Theme> = text(&self.display_buffer)
			.size(40)
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
				.size(25);

			let button = button(key_label)
				.on_press(Message::Click(key.clone()))
				.width(iced::Length::Fill)
				.height(iced::Length::Fill);

			current_row.push(button.into());

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
			.align_x(alignment::Horizontal::Center);

		center(content).into()
	}
}
