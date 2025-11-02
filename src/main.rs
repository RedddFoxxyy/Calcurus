//! The Basic Working and logic of operation of this calculator is that, there is an empty buffer where we can push and pop Arithmetic Units ( see types.rs ).
//! When we type any number/operator on the calculator, these values are pushed to this buffer and during this various edge cases are also checked.
//! Once the user clicks the 'equal to' key, instead of pushing this to buffer, a function is called to operate on this buffer, and this function operates
//! on each unit one by one.
//!
//! To handle a number with multiple digits, a number is only pushed to the buffer once the user clicks on an operator or when equal to sign is clicked.
//! However, this does not affect the handling of the visual buffer (the display of calculator). The clicked key character gets pushed to the visual buffer immediately.

// TODO: Handle NaN and Out of Bounds.

mod parser;

use iced::{
	Element, Font, Size, Theme, alignment,
	widget::{button, center, column, row, text},
	window,
};

static JETBRAINS_MONO_BYTES: &[u8] = include_bytes!("./resources/fonts/JetBrainsMonoNerdFont-Regular.ttf");
pub const JETBRAINS_MONO_NERD_FONT: Font = Font::with_name("JetBrainsMono Nerd Font");

#[derive(Debug, Clone)]
pub(crate) enum Message {
	Click(String),
}

struct Calcurus {
	debug_mode: bool,
	/// Buffer to store the string that will be displayed on the calculator.
	display_buffer: String,
	/// Used to track if the output of last operation was decimal value or not.
	is_output_dec: bool,
	/// Parser unit that will be used to operate on the buffer.
	parser: parser::AUParser,
	/// Stores the Keyboard Keys.
	keyboard: Vec<&'static str>,
}

impl Default for Calcurus {
	fn default() -> Self {
		let keys: Vec<&'static str> = generate_key_layout();

		Self {
			debug_mode: false,
			display_buffer: String::new(),
			parser: parser::AUParser::init(),
			// thought [initialization]: Should this be initialized as true or not?
			is_output_dec: true,
			keyboard: keys,
		}
	}
}

impl Calcurus {
	pub(crate) fn update(&mut self, message: Message) {
		match message {
			Message::Click(button_id) => handle_key_click(self, button_id),
		}
	}
	pub(crate) fn view(&self) -> Element<'_, Message> {
		let display: iced::widget::Text<Theme> = text(&self.display_buffer)
			.size(30)
			.width(iced::Length::Fill)
			.height(iced::Length::FillPortion(1))
			.align_x(alignment::Horizontal::Right);

		let button_rows = create_default_rows(self);

		let keys_column: iced::widget::Column<_> = column(button_rows).spacing(3).height(iced::Length::FillPortion(4));

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
		"7", "8", "9", "+", "4", "5", "6", "-", "1", "2", "3", "×", "0", ".", "^", "÷", "√", "Bck", "Clr", "=",
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
		// TODO: Add handling case for '√'
		'0'..='9' | '.' | '+' | '-' | '*' | '×' | '/' | '÷' | '^' => {
			state.display_buffer.push(button_id_char);
		}
		'=' => {
			operate_on_buffer(state);
		}
		_ => (),
	}
}

fn handle_delete_keys(state: &mut Calcurus, button_id: &str) {
	if button_id == "Clr" {
		state.display_buffer.clear();
		state.is_output_dec = true;
	} else if button_id == "Bck" {
		if state.is_output_dec {
			state.display_buffer.pop();
		} else {
			state.display_buffer.clear();
			state.is_output_dec = true;
		}
	}
}

fn operate_on_buffer(app_state: &mut Calcurus) {
	app_state.parser.set_input(app_state.display_buffer.clone());
	let result = app_state.parser.calculate_result();
	app_state.display_buffer.clear();
	app_state.display_buffer.push_str(result.to_string().as_str());
	app_state.parser.reset();
	app_state.is_output_dec = true;
}

#[allow(unused)]
fn set_gpu_backend() {
	if cfg!(target_os = "windows") {
		unsafe {
			std::env::set_var("WGPU_BACKEND", "dx12");
		}
	} else if cfg!(target_os = "linux") {
		unsafe {
			std::env::set_var("WGPU_BACKEND", "vulkan");
		}
	} else if cfg!(target_os = "macos") {
		unsafe {
			std::env::set_var("WGPU_BACKEND", "metal");
		}
	} else {
		// Set a default backend or handle unsupported OS
		unsafe {
			std::env::set_var("ICED_BACKEND", "tiny-skia");
			// std::env::set_var("WGPU_BACKEND", "vulkan");
		}
		eprintln!("Warning: Operating system not specifically handled. Using Software Rendering.");
	}
}

static MIN_WINDOW_SIZE: Size = Size { width: 280.0, height: 400.0 };

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
