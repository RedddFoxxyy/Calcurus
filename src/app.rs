use cosmic::Element;
use cosmic::iced::widget::{column, row};
use cosmic::iced::{
	Color, Length,
	alignment::{Horizontal, Vertical},
};
use cosmic::iced_widget::{button, center, text};

use crate::libcalcurus;

#[rustfmt::skip]
static CL_BUTTONS_LAYOUT: [&str; 20] = [
	"7", "8", "9", "+",
	"4", "5", "6", "-",
	"1", "2", "3", "×",
	"0", ".", "^", "÷",
	"√", "Bck", "Clr", "=",
];

/// Custom messages for Calcurus.
#[derive(Debug, Clone)]
pub enum ClMessage {
	ClClick(String),
}

pub struct ClApplication {
	core: cosmic::app::Core,
	debug_mode: bool,
	display_buffer: String,
	// Used to track if the output of last operation was an error or not.
	last_operation_success: bool,
	parser: libcalcurus::ShuntParser,
}

impl cosmic::Application for ClApplication {
	type Executor = cosmic::executor::Default;

	type Flags = ();

	type Message = ClMessage;

	/// The unique application ID to supply to the window manager.
	const APP_ID: &'static str = "io.github.redddfoxxyy.calcurus";

	fn core(&self) -> &cosmic::app::Core {
		&self.core
	}

	fn core_mut(&mut self) -> &mut cosmic::app::Core {
		&mut self.core
	}

	/// Creates the application, and optionally emits task on initialize.
	fn init(core: cosmic::app::Core, _flags: Self::Flags) -> (Self, cosmic::app::Task<Self::Message>) {
		// let nav_model = nav_bar::Model::default();

		let app = ClApplication {
			core,
			debug_mode: false,
			display_buffer: String::new(),
			parser: libcalcurus::ShuntParser::new(),
			// thought [initialization]: Should this be initialized as true or not?
			last_operation_success: true,
		};

		let command = cosmic::app::Task::none();

		(app, command)
	}

	fn update(&mut self, message: ClMessage) -> cosmic::app::Task<Self::Message> {
		match message {
			ClMessage::ClClick(button_id) => self.handle_key_click(button_id),
		}
		cosmic::app::Task::none()
	}

	fn view(&self) -> Element<'_, Self::Message> {
		let display = text(&self.display_buffer)
			.size(30)
			.width(Length::Fill)
			.height(Length::FillPortion(1))
			.align_x(Horizontal::Right);

		let button_rows = self.create_default_rows();

		let keys_column = column(button_rows).spacing(3).height(Length::FillPortion(4));

		let content = column![display, keys_column]
			.padding(5)
			.spacing(5)
			.width(Length::Fill)
			.align_x(Horizontal::Center);

		let mut main_content: Element<ClMessage> = center(content).into();
		if self.debug_mode {
			main_content = main_content.explain(Color::WHITE);
		}
		main_content
	}
}

impl ClApplication
where
	Self: cosmic::Application,
{
	pub(crate) fn handle_key_click(&mut self, button_id: String) {
		self.handle_delete_keys(&button_id);

		// Handle Error on last operation.
		if !self.last_operation_success {
			self.display_buffer.clear();
			self.last_operation_success = true;
		}

		let button_id_char = button_id.chars().next().unwrap();
		match button_id_char {
			// TODO: Add handling case for '√'
			'0'..='9' | '.' | '+' | '-' | '*' | '×' | '/' | '÷' | '^' => {
				self.display_buffer.push(button_id_char);
			}
			'=' => {
				self.operate_on_buffer();
			}
			_ => (),
		}
	}

	fn handle_delete_keys(&mut self, button_id: &str) {
		if button_id == "Clr" {
			self.display_buffer.clear();
			self.last_operation_success = true;
		} else if button_id == "Bck" {
			// Handle Error on last operation.
			if self.last_operation_success {
				self.display_buffer.pop();
			} else {
				self.display_buffer.clear();
				self.last_operation_success = true;
			}
		}
	}

	fn operate_on_buffer(&mut self) {
		self.parser.set_input(self.display_buffer.clone());

		match self.parser.calculate_result() {
			Ok(result) => {
				self.display_buffer.clear();
				self.display_buffer.push_str(result.to_string().as_str());
				self.parser.reset();
				self.last_operation_success = true;
			}
			Err(error) => {
				self.display_buffer.clear();
				self.display_buffer.push_str(error.as_str());
				self.parser.reset();
				self.last_operation_success = false;
			}
		}
	}

	fn create_default_rows(&self) -> Vec<Element<'_, ClMessage>> {
		let mut button_rows: Vec<Element<ClMessage>> = Vec::new();
		let mut current_row: Vec<Element<ClMessage>> = Vec::new();

		for (index, key) in CL_BUTTONS_LAYOUT.iter().enumerate() {
			// let key_label = text(*key)
			// 	.width(Length::Fill)
			// 	.height(Length::Fill)
			// 	.align_x(alignment::Horizontal::Center)
			// 	.align_y(alignment::Vertical::Center)
			// 	// .color(iced::Color::new(0.9490196, 0.8980392, 0.7372549, 1.0))
			// 	.size(25);

			let button = cosmic::widget::button::standard(*key).on_press(ClMessage::ClClick(key.to_string()));
			// .width(Length::Fill)
			// .height(Length::Fill);

			let mut button_element: Element<ClMessage> = button.into();
			if self.debug_mode {
				button_element = button_element.explain(Color::WHITE);
			}

			current_row.push(button_element);

			// Create a new row after every 4 buttons
			if current_row.len() == 4 || index == CL_BUTTONS_LAYOUT.len() - 1 {
				button_rows.push(
					cosmic::widget::container(
						cosmic::widget::flex_row::flex_row(std::mem::take(&mut current_row))
							.column_spacing(4)
							.row_spacing(4),
					)
					.width(Length::Fill)
					.height(Length::Fill)
					.align_x(Horizontal::Center)
					.align_y(Vertical::Center)
					.into(), // row().spacing(3).into()
				);
			}
		}
		button_rows
	}
}
