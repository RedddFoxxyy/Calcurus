use calcurus::state::*;
use iced::{window, Size, Theme};

mod calcurus;

const MIN_WINDOW_SIZE: Size = Size {
	width: 280.0,
	height: 400.0,
};

fn main() -> iced::Result {
	let settings = window::Settings {
		size: MIN_WINDOW_SIZE,
		min_size: Some(MIN_WINDOW_SIZE),
		..window::Settings::default()
	};

	iced::application("Calcurus - Iced", Calcurus::update, Calcurus::view)
		.window(settings)
		.theme(|_| Theme::Ferra)
		.run()
}
