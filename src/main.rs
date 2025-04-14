use calcurus::state::*;
use iced::{Size, Theme, window};
use std::env;

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
		.theme(|_| Theme::GruvboxDark)
		.run()
}

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
