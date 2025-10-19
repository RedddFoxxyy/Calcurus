use std::env;

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