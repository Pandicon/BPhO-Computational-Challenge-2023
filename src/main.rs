pub mod application;
pub mod constants;
pub mod enums;
pub mod input;
mod rendering;
pub mod structs;

fn main() {
	let native_options = eframe::NativeOptions {
		maximized: true,
		resizable: true,
		icon_data: Some(eframe::IconData::try_from_png_bytes(&std::fs::read("./icon.png").expect("Icon file not found!")).expect("Icon file is not png!")),
		..Default::default()
	};

	eframe::run_native("BPhO Computational Challenge 2023", native_options, Box::new(|cc| Box::new(application::Application::init(cc)))).expect("Failed to start the application");
}
