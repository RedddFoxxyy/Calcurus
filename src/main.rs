// use iced::{
// 	Font, Size, Theme, color,
// 	theme::{Custom, Palette},
// 	window,
// };

use cosmic::iced::{Font, Size};

mod app;
mod config;
mod i18n;
mod libcalcurus;

#[allow(unused)]
static JETBRAINS_MONO_BYTES: &[u8] = include_bytes!("../resources/fonts/JetBrainsMonoNerdFont-Regular.ttf");
#[allow(unused)]
static WINDOW_ICON_BYTES: &[u8] = include_bytes!("../resources/icons/hicolor/4000x4000/apps/calcurus.png");
pub const JETBRAINS_MONO_NERD_FONT: Font = Font::with_name("JetBrainsMono Nerd Font");
#[allow(unused)]
static MIN_WINDOW_SIZE: cosmic::iced::Size = Size { width: 280.0, height: 400.0 };

fn main() -> cosmic::iced::Result {
	// let icon = window::icon::from_file_data(WINDOW_ICON_BYTES, None).ok();

	// pub const KANAGAWA_DRAGON: Palette = Palette {
	// 	background: color!(0x181616), // Dragon Black 3
	// 	text: color!(0xc5c9c5),       // Dragon White
	// 	primary: color!(0x96313a),    // Custom Red
	// 	success: color!(0x8a9a7b),    // Dragon Green 2
	// 	danger: color!(0xc4746e),     // Dragon Red
	// };

	// Get the system's preferred languages.
	let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

	// Enable localizations to be applied.
	i18n::init(&requested_languages);

	// let window_settings = window::Settings {
	// 	size: MIN_WINDOW_SIZE,
	// 	min_size: Some(MIN_WINDOW_SIZE),
	// 	icon,
	// 	..window::Settings::default()
	// };

	let settings = cosmic::app::Settings::default().size_limits(cosmic::iced::Limits::NONE.min_width(400.0).min_height(600.0));

	// iced::application("Calcurus - Iced", app::Calcurus::update, app::Calcurus::view)
	// 	.font(JETBRAINS_MONO_BYTES)
	// 	.default_font(JETBRAINS_MONO_NERD_FONT)
	// 	.window(window_settings)
	// 	.theme(|_| Theme::Custom(std::sync::Arc::new(Custom::new("Calcurus".to_string(), KANAGAWA_DRAGON))))
	// 	.run()

	cosmic::app::run::<app::ClApplication>(settings, ())
}
