use calcurus::state::*;
use iced::{window, Font, Size, Theme};
mod calcurus;


static JETBRAINS_MONO_BYTES: &[u8] = include_bytes!("./resources/fonts/JetBrainsMonoNerdFont-Regular.ttf");
pub const JETBRAINS_MONO_NERD_FONT: Font = Font::with_name("JetBrainsMono Nerd Font");


const MIN_WINDOW_SIZE: Size = Size {
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


