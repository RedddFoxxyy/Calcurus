//! The Basic Working and logic of operation of this calculator is that, there is an empty buffer where we can push and pop Arithmetic Units ( see types.rs ).
//! When we type any number/operator on the calculator, these values are pushed to this buffer and during this various edge cases are also checked.
//! Once the user clicks the 'equal to' key, instead of pushing this to buffer, a function is called to operate on this buffer, and this function operates
//! on each unit one by one.
//!
//! To handle a number with multiple digits, a number is only pushed to the buffer once user clicks on an operator or when equal to sign is clicked.
//! However, this does not affect the handling of visual buffer ( the display of calculator ). The clicked key character gets pushed to visual buffer immediately.

// TODO: First handle inputs directly into the buffer, and then do the operation on all the units one by one.

use std::env;
use iced::{alignment, window, Element, Font, Size, Theme};
use iced::widget::{button, center, column, row, text};
use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;


static JETBRAINS_MONO_BYTES: &[u8] = include_bytes!("./resources/fonts/JetBrainsMonoNerdFont-Regular.ttf");
pub const JETBRAINS_MONO_NERD_FONT: Font = Font::with_name("JetBrainsMono Nerd Font");

/// An Arithmetic Unit type, is an enum that can have two types either a DecNumber or an Operator.
/// Where an Operator is any arithmetic operator such as '+' and, a Number is of type 'Decimal' from
/// the 'rust_decimal' crate.
#[derive(Clone)]
pub enum ArithmeticUnit {
	Number(Decimal),
	Operator(String),
}

/// A Buffer(Vector) of ArithmeticUnits, and the current selected/entered unit.
#[derive(Default)]
pub struct ArithmeticUnitBuffer {
	pub buffer: Vec<ArithmeticUnit>,
	pub current_unit: Option<ArithmeticUnit>,
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
	Click(String),
}

pub(crate) struct Calcurus {
	pub debug_mode: bool,
	/// Buffer to store NumObjects to Operate on.
	pub unit_buf: ArithmeticUnitBuffer,
	/// Buffer to store the string that will be displayed on the calculator.
	pub display_buffer: String,
	/// Temporary buffer to store the number that is being typed.
	pub current_input_buffer: String,
	/// Used to track if the output of last operation was decimal value or not.
	pub is_output_dec: bool,
	/// Stores the Keyboard Keys.
	pub keyboard: Vec<&'static str>,
}

impl Default for Calcurus {
	fn default() -> Self {
		let keys: Vec<&'static str> = generate_key_layout();

		Self {
			debug_mode: false,
			unit_buf: ArithmeticUnitBuffer::default(),
			display_buffer: String::new(),
			// thought [initialization]: Should this be initialized as true or not?
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
	pub(crate) fn view(&self) -> Element<'_, Message> {
		// println!("Number of buttons in keyboard: {}", self.state.keyboard.len());
		let display: iced::widget::Text<Theme> = text(&self.display_buffer)
			.size(30)
			.width(iced::Length::Fill)
			.height(iced::Length::FillPortion(1))
			.align_x(alignment::Horizontal::Right);

		let button_rows = create_default_rows(self);

		let keys_column: iced::widget::Column<_> = column(button_rows)
			.spacing(3)
			.height(iced::Length::FillPortion(4));

		let content: iced::widget::Column<_> = iced::widget::column![display, keys_column]
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

pub fn generate_key_layout() -> Vec<&'static str> {
	vec![
		"7",
		"8",
		"9",
		"+",
		"4",
		"5",
		"6",
		"-",
		"1",
		"2",
		"3",
		"×",
		"0",
		".",
		"^",
		"÷",
		"√",
		"Bck",
		"Clr",
		"=",
	]
}

pub(crate) fn create_default_rows(calcurus: &'_ Calcurus) -> Vec<Element<'_, Message>> {
	let mut button_rows: Vec<Element<Message>> = Vec::new();
	let mut current_row: Vec<Element<Message>> = Vec::new();

	// Iterate through all buttons in the keyboard
	for (index, key) in calcurus.keyboard.iter().enumerate() {
		let key_label = text(*key)
			.width(iced::Length::Fill)
			.height(iced::Length::Fill)
			.align_x(alignment::Horizontal::Center)
			.align_y(alignment::Vertical::Center)
			.color(iced::Color::new(0.9490196, 0.8980392, 0.7372549, 1.0))
			.size(25);

		let button = button(key_label)
			.on_press(Message::Click(key.to_string()))
			.width(iced::Length::Fill)
			.height(iced::Length::Fill);

		let mut button_element: Element<Message> = button.into();
		if calcurus.debug_mode {
			button_element = button_element.explain(iced::Color::WHITE);
		}

		current_row.push(button_element);

		// Create a new row after every 4 buttons
		if current_row.len() == 4 || index == calcurus.keyboard.len() - 1 {
			button_rows.push(row(std::mem::take(&mut current_row)).spacing(3).into());
		}
	}
	button_rows
}

pub(crate) fn handle_key_click(state: &mut Calcurus, button_id: String) {
	handle_delete_keys(state, &button_id);

	// TODO: Replace unwrap with unwrap_or_else.
	let button_id_char = button_id.chars().next().unwrap();
	match button_id_char {
		'0'..='9' | '.' => handle_num_keys(state, button_id_char),

		// TODO: Add handling case for '√'
		'+' | '-' | '×' | '÷' | '^' => {
			// If the num_string_buffer is empty, only allow input of + or - to give
			// sign to the number.
			if state.current_input_buffer.is_empty() {
				if button_id_char == '+' || button_id_char == '-' {
					state.push_current_input(&button_id_char);
				}
				return;
			}
			// Get the current number ( as Decimal ) present in the num string buffer before clearing the
			// num_string_buffer for the next number after the operator.
			let new_num: Decimal = state.current_input_buffer.parse().unwrap();
			// Clear the num_string_buffer for the next number.
			state.current_input_buffer.clear();
			// Depending upon the presence of current_object, make the current number as the current object if current
			// object is none else push the current object to num_buffer, push the operator to num buffer and then make
			// the new_num as the current object
			if state.unit_buf.current_unit.is_none() {
				state.unit_buf.current_unit = Some(ArithmeticUnit::Number(new_num));
				let operator = ArithmeticUnit::Operator(button_id);
				state.unit_buf.buffer.push(operator);
				state.display_buffer.push(button_id_char);
			} else {
				let current_num_object = state.unit_buf.current_unit.clone().unwrap();
				state.unit_buf.current_unit = None;
				let operator = ArithmeticUnit::Operator(button_id);
				state.unit_buf.buffer.push(current_num_object);
				state.unit_buf.buffer.push(operator);
				state.display_buffer.push(button_id_char);
			}
		}
		'=' => {
			// Parse and add the current number to num_buffer before operating
			if !state.current_input_buffer.is_empty() {
				let final_num = state.current_input_buffer.parse::<Decimal>().unwrap();
				state.current_input_buffer.clear();

				let num_object = ArithmeticUnit::Number(final_num);
				if state.unit_buf.current_unit.is_some() {
					let current_obj = state.unit_buf.current_unit.take().unwrap();
					state.unit_buf.buffer.push(current_obj);
				}
				state.unit_buf.buffer.push(num_object);
			}
			operate_on_buffer(state);
			if state.unit_buf.buffer.len() == 1 {
				let current_num_object: ArithmeticUnit = state.unit_buf.buffer[0].clone();

				if let ArithmeticUnit::Number(current_num) = current_num_object {
					let current_num_string = current_num.to_string();
					state.current_input_buffer.push_str(&current_num_string);
				}

				state.unit_buf.buffer.clear();
			}
		}

		_ => (),
	}
}
fn handle_delete_keys(state: &mut Calcurus, button_id: &str) {
	if button_id == "Clr" {
		state.unit_buf.buffer.clear();
		state.display_buffer.clear();
		state.current_input_buffer.clear();
		state.is_output_dec = true;
	} else if button_id == "Bck" {
		if state.is_output_dec {
			if state.current_input_buffer.is_empty() {
				state.unit_buf.buffer.pop();
				state.display_buffer.pop();
			} else {
				state.display_buffer.pop();
				state.current_input_buffer.pop();
			}
		} else {
			state.unit_buf.buffer.clear();
			state.display_buffer.clear();
			state.current_input_buffer.clear();
			state.is_output_dec = true;
		}
	}
}

fn handle_num_keys(state: &mut Calcurus, button_id_char: char) {
	if !state.is_output_dec {
		state.unit_buf.buffer.clear();
		state.display_buffer.clear();
		state.current_input_buffer.clear();
		state.push_current_input(&button_id_char);
		state.is_output_dec = true;
	} else {
		state.push_current_input(&button_id_char);
	}
}

fn operate_on_buffer(app_state: &mut Calcurus) {
	let mut first_num: bool = true;
	let mut buf1: Decimal = dec!(0);
	let mut buf2: Decimal;

	let mut current_operator: ArithmeticUnit = ArithmeticUnit::Operator("+".to_string());
	let num_object_iterator = app_state.unit_buf.buffer.iter();

	for num_object in num_object_iterator {
		if let &ArithmeticUnit::Number(num) = num_object {
			if first_num {
				buf1 = num;
				first_num = false;
			} else {
				buf2 = num;
				app_state.is_output_dec = perform_calculation(
					&mut buf1,
					&mut buf2,
					&mut current_operator,
					&mut app_state.display_buffer,
				);
			}
		} else {
			current_operator = num_object.clone();
		}
	}

	let buf1_string = buf1.to_string();
	if app_state.is_output_dec {
		let buf1_dec = buf1_string.parse::<Decimal>().unwrap();
		let buf1_num_object = ArithmeticUnit::Number(buf1_dec);

		app_state.unit_buf.buffer.clear();
		app_state.unit_buf.buffer.push(buf1_num_object);
		// num_object_buffer.current_object = Some(buf1_num_object);

		app_state.display_buffer.clear();
		app_state.display_buffer.push_str(&buf1_string);
	}
}

fn perform_calculation(
	buf1: &mut Decimal,
	buf2: &mut Decimal,
	operator: &mut ArithmeticUnit,
	display_buffer: &mut String,
) -> bool {
	let operator_value = match operator {
		ArithmeticUnit::Operator(operator_value_inner) => operator_value_inner.clone(),
		_ => unreachable!(),
	};

	match operator_value.as_str() {
		"+" => *buf1 += *buf2,
		"-" => *buf1 -= *buf2,
		"×" => *buf1 *= *buf2,
		"÷" => {
			if *buf2 == dec!(0) {
				display_buffer.clear();
				display_buffer.push_str("Cannot Divide By 0!");
				return false;
			}
			*buf1 /= *buf2
		}
		"^" => {
			let temp_dec = *buf1;
			let output_check = temp_dec.checked_powd(*buf2);
			if output_check.is_none() {
				display_buffer.clear();
				display_buffer.push_str("Value out of bounds!");
				return false;
			}
			*buf1 = output_check.unwrap();
		}
		_ => unreachable!(),
	}
	true
}

#[allow(unused)]
fn set_gpu_backend() {
	if cfg!(target_os = "windows") {
		unsafe {
			env::set_var("WGPU_BACKEND", "dx12");
		}
	} else if cfg!(target_os = "linux") {
		unsafe {
			env::set_var("WGPU_BACKEND", "vulkan");
		}
	} else if cfg!(target_os = "macos") {
		unsafe {
			env::set_var("WGPU_BACKEND", "metal");
		}
	} else {
		// Set a default backend or handle unsupported OS
		unsafe {
			env::set_var("ICED_BACKEND", "tiny-skia");
			// env::set_var("WGPU_BACKEND", "vulkan"); // Default to vulkan
		}
		eprintln!("Warning: Operating system not specifically handled. Using Software Rendering.");
	}
}

static MIN_WINDOW_SIZE: Size = Size {
	width: 280.0,
	height: 400.0,
};

fn main() -> iced::Result {
	let window_settings = window::Settings {
		size: MIN_WINDOW_SIZE,
		min_size: Some(MIN_WINDOW_SIZE),
		..window::Settings::default()
	};

	iced::application("Calcurus - Iced", Calcurus::update, Calcurus::view)
		.font(JETBRAINS_MONO_BYTES)
		.default_font(JETBRAINS_MONO_NERD_FONT)
		.window(window_settings)
		.theme(|_| Theme::KanagawaDragon)
		.run()
}


