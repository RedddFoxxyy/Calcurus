use crate::libcalcurus;
// use iced::{
// 	Element, Theme, alignment,
// 	widget::{button, center, column, row, text},
// };

#[rustfmt::skip]
static KEYBOARD_LAYOUT: [&str; 20] = [
	"7", "8", "9", "+",
	"4", "5", "6", "-",
	"1", "2", "3", "×",
	"0", ".", "^", "÷",
	"√", "Bck", "Clr", "=",
];

#[derive(Debug, Clone)]
pub(crate) enum Message {
	Click(String),
}

pub struct Calcurus {
	debug_mode: bool,
	/// Buffer to store the string that will be displayed on the calculator.
	display_buffer: String,
	/// Used to track if the output of last operation was decimal value or not.
	is_output_dec: bool,
	/// Parser unit that will be used to operate on the buffer.
	parser: parser::ShuntParser,
}

impl Default for Calcurus {
	fn default() -> Self {
		Self {
			debug_mode: false,
			display_buffer: String::new(),
			parser: parser::ShuntParser::new(),
			// thought [initialization]: Should this be initialized as true or not?
			is_output_dec: true,
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

pub(crate) fn create_default_rows(calcurus: &'_ Calcurus) -> Vec<Element<'_, Message>> {
	let mut button_rows: Vec<Element<Message>> = Vec::new();
	let mut current_row: Vec<Element<Message>> = Vec::new();

	// Iterate through all buttons in the keyboard
	for (index, key) in KEYBOARD_LAYOUT.iter().enumerate() {
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
		if current_row.len() == 4 || index == KEYBOARD_LAYOUT.len() - 1 {
			button_rows.push(row(std::mem::take(&mut current_row)).spacing(3).into());
		}
	}
	button_rows
}

pub(crate) fn handle_key_click(state: &mut Calcurus, button_id: String) {
	handle_delete_keys(state, &button_id);

	// Handle Error on last operation.
	if !state.is_output_dec {
		state.display_buffer.clear();
		state.is_output_dec = true;
	}

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
		// Handle Error on last operation.
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

	match app_state.parser.calculate_result() {
		Ok(result) => {
			app_state.display_buffer.clear();
			app_state.display_buffer.push_str(result.to_string().as_str());
			app_state.parser.reset();
			app_state.is_output_dec = true;
		}
		Err(error) => {
			app_state.display_buffer.clear();
			app_state.display_buffer.push_str(error.as_str());
			app_state.parser.reset();
			app_state.is_output_dec = false;
		}
	}
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
