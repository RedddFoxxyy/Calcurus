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
static MIN_WINDOW_SIZE: cosmic::iced::Size = Size { width: 330.0, height: 520.0 };

fn main() -> cosmic::iced::Result {
	// let icon = window::icon::from_file_data(WINDOW_ICON_BYTES, None).ok();

	// Get the system's preferred languages.
	let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

	// Enable localizations to be applied.
	i18n::init(&requested_languages);

	let settings = cosmic::app::Settings::default()
		.size_limits(cosmic::iced::Limits::NONE.min_width(350.0).min_height(560.0))
		.antialiasing(true)
		.autosize(true)
		.default_font(JETBRAINS_MONO_NERD_FONT)
		.size(MIN_WINDOW_SIZE);

	cosmic::app::run::<app::ClApplication>(settings, ())
}
