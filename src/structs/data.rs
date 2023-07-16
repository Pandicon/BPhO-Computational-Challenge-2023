use std::collections::HashMap;

use eframe::epaint::Color32;

pub struct Data {
	pub task_1_data: Task1Data,
}

impl Data {
	pub fn new() -> Self {
		Self { task_1_data: Task1Data::new() }
	}

	pub fn init_task_1(&mut self, planetary_system: &crate::structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		self.task_1_data.init(planetary_system, active_groups);
	}
}

pub struct Task1Data {
	pub points: Vec<(f64, f64, Color32)>,
}

impl Task1Data {
	pub fn new() -> Self {
		Self { points: Vec::new() }
	}

	fn init(&mut self, planetary_system: &crate::structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		let mut points = Vec::new();
		for object in &planetary_system.objects {
			if !*active_groups.get(&object.group).unwrap_or(&true) {
				continue;
			}
			points.push((object.distance_au.powf(1.5), object.period_years, object.colour));
		}
		self.points = points;
	}
}
