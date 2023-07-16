pub mod application;
pub mod constants;
pub mod enums;
mod rendering;
pub mod structs;

fn main() {
	let native_options = eframe::NativeOptions {
		maximized: true,
		resizable: true,
		..Default::default()
	};

	eframe::run_native("BPhO CC 2023", native_options, Box::new(|cc| Box::new(application::Application::init(cc)))).expect("Failed to start the application");
}
