use cosmic::Element;
use cosmic::iced::Padding;
use cosmic::iced::{
	Length,
	alignment::{Horizontal, Vertical},
};
use cosmic::widget;

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
	ClClick(&'static str),
}

pub struct ClApplication {
	core: cosmic::app::Core,
	#[allow(unused)]
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
		widget::column::column()
			.push(
				widget::container(widget::text(&self.display_buffer).size(40).width(Length::Fill).align_x(Horizontal::Right))
					.width(Length::Fill)
					.align_x(Horizontal::Center)
					.padding(10),
			)
			.push(widget::divider::horizontal::default())
			.push(
				widget::container(init_keys_grid())
					.width(Length::Fill)
					.height(Length::Fill)
					.align_x(Horizontal::Center)
					.align_y(Vertical::Bottom)
					.padding(Padding {
						top: 0.0,
						bottom: 20.0,
						left: 2.5,
						right: 2.5,
					}),
			)
			.padding(5)
			.spacing(5)
			.width(Length::Fill)
			.align_x(Horizontal::Center)
			.into()
	}
}

impl ClApplication
where
	Self: cosmic::Application,
{
	pub(crate) fn handle_key_click(&mut self, button_id: &'static str) {
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
}

fn init_keys_grid() -> Element<'static, ClMessage> {
	let grid = widget::grid();

	let grid = CL_BUTTONS_LAYOUT.chunks(4).enumerate().fold(grid, |grid, (i, row_keys)| {
		let grid = if i > 0 { grid.insert_row() } else { grid };

		row_keys.iter().fold(grid, |grid, key| grid.push(create_button(key)))
	});

	grid.column_spacing(10).row_spacing(10).into()
}

fn create_button(key: &'static str) -> Element<'static, ClMessage> {
	cosmic::widget::button::standard(key)
		.on_press(ClMessage::ClClick(key))
		.class(if key == "=" {
			cosmic::theme::Button::Suggested
		} else {
			cosmic::theme::Button::Standard
		})
		.font_size(24)
		.width(60)
		.height(60)
		.into()
}
