#[allow(unused_imports)]
use crate::calcurus::{
	defines::{NumObject, NumObjectBuffer},
	keys::*,
	logic::handle_key_click,
};

#[derive(Debug, Clone)]
pub(crate) enum Message {
	Click(String),
}

pub(crate) struct Calcurus {
	pub debug_mode: bool,
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
			debug_mode: false,
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
}
