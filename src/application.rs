use std::{collections::HashMap, fs};

use eframe::{self, egui};

use crate::{
	enums::{self, Task},
	structs,
};

const PLANETARY_SYSTEMS_NAMES_FILE: &str = "./data/planetary-systems-names.csv";
const PLANETARY_SYSTEMS_FOLDER: &str = "./data/planetary-systems";

pub struct Application {
	/// active groups for a given system within a given task: planetary_systems[task_index][system_index]
	pub active_groups: Vec<Vec<HashMap<String, bool>>>,
	pub planetary_systems: Vec<structs::PlanetarySystem>,
	pub chosen_system: usize,
	pub chosen_task: enums::Task,
	pub data: structs::Data,
	pub show_loaded_systems: bool,
}

impl Application {
	pub fn init(cc: &eframe::CreationContext<'_>) -> Self {
		cc.egui_ctx.set_visuals(egui::Visuals::dark());

		let chosen_system = 0;

		let mut planetary_systems_names = HashMap::new();
		if let Ok(mut reader) = csv::Reader::from_path(PLANETARY_SYSTEMS_NAMES_FILE) {
			for name_data in reader.deserialize::<[String; 2]>().flatten() {
				let [filename, name] = name_data;
				planetary_systems_names.insert(filename, name);
			}
		}

		let mut planetary_systems = Vec::new();
		if let Ok(files) = fs::read_dir(PLANETARY_SYSTEMS_FOLDER) {
			for file in files.flatten() {
				let path = file.path();
				let file_name = path.file_name();
				if file_name.is_none() {
					continue;
				}
				let file_name = file_name.unwrap().to_str();
				if file_name.is_none() {
					continue;
				}
				let file_name = file_name.unwrap().to_string();

				let mut system_objects = Vec::new();
				if let Ok(mut reader) = csv::Reader::from_path(file.path()) {
					for planetary_object_raw in reader.deserialize::<structs::PlanetaryObjectRaw>().flatten() {
						system_objects.push(structs::PlanetaryObject::from_raw(planetary_object_raw));
					}
				}
				if !system_objects.is_empty() {
					let name = if let Some(name) = planetary_systems_names.get(&file_name) { name.to_owned() } else { file_name };
					planetary_systems.push(structs::PlanetarySystem::new(system_objects, name));
				}
			}
		}
		if planetary_systems.is_empty() {
			panic!("No planetary systems could be loaded");
		}
		planetary_systems.sort_by(|a, b| b.name.cmp(&a.name));
		let mut active_groups_per_task = Vec::new();
		for planetary_system in &planetary_systems {
			let mut active_groups_system = HashMap::new();
			for object in &planetary_system.objects {
				active_groups_system.insert(object.group.to_owned(), true);
			}
			active_groups_per_task.push(active_groups_system);
		}

		let active_groups = vec![active_groups_per_task; crate::enums::TASKS_NUM];

		let mut data = structs::Data::new();
		for task_i in 0..crate::enums::TASKS_NUM {
			data.init_task_by_id(task_i, chosen_system, &planetary_systems, &active_groups);
		}

		Self {
			active_groups,
			planetary_systems,
			chosen_system,
			chosen_task: Task::Task1,
			data,
			show_loaded_systems: false,
		}
	}
}

impl eframe::App for Application {
	fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
		let input = self.get_input(ctx);
		self.handle_input(input);
		self.render_loaded_systems_window(ctx);
		/*match self.chosen_task {
			Task::Task1 | Task::Task2 | Task::Task2Rotated => ctx.tessellation_options_mut(|tess_options| {
				tess_options.feathering = true;
			}),
			Task::Task4 => ctx.tessellation_options_mut(|tess_options| {
				tess_options.feathering = false;
			}),
		}*/
		if self.chosen_task.render_after_top_panel() {
			self.data.top_panel_bottom = self.render_top_panel(ctx).response.rect.max.y;
		}
		match self.chosen_task {
			Task::Task1 => self.render_task_1(ctx),
			Task::Task2 => self.render_task_2(ctx),
			Task::Task2Rotated => self.render_task_2_rotated(ctx),
			Task::Task4 => self.render_task_4(ctx),
		}
		if !self.chosen_task.render_after_top_panel() {
			self.data.top_panel_bottom = self.render_top_panel(ctx).response.rect.max.y;
		}
		ctx.request_repaint();
	}
}
