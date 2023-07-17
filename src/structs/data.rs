use std::{
	collections::HashMap,
	f64::consts::{PI, TAU},
};

use eframe::epaint::Color32;

use crate::{constants, enums, structs};

pub struct Data {
	pub task_1_data: Task1Data,
	pub task_2_data: Task2Data,
	pub task_2_rotated_data: Task2RotatedData,
}

impl Data {
	pub fn new() -> Self {
		Self {
			task_1_data: Task1Data::new(),
			task_2_data: Task2Data::new(),
			task_2_rotated_data: Task2RotatedData::new(),
		}
	}

	pub fn init_task(&mut self, chosen_task: &enums::Task, chosen_system: usize, planetary_systems: &Vec<structs::PlanetarySystem>, active_groups: &Vec<Vec<HashMap<String, bool>>>) {
		match *chosen_task {
			enums::Task::Task1 => self.init_task_1(&planetary_systems[chosen_system], &active_groups[chosen_task.task_index()][chosen_system]),
			enums::Task::Task2 => self.init_task_2(&planetary_systems[chosen_system], &active_groups[chosen_task.task_index()][chosen_system]),
			enums::Task::Task2Rotated => self.init_task_2_rotated(&planetary_systems[chosen_system], &active_groups[chosen_task.task_index()][chosen_system]),
		}
	}

	pub fn init_task_by_id(&mut self, chosen_task: usize, chosen_system: usize, planetary_systems: &Vec<structs::PlanetarySystem>, active_groups: &Vec<Vec<HashMap<String, bool>>>) {
		self.init_task(&enums::Task::from_index(chosen_task), chosen_system, planetary_systems, active_groups);
	}

	fn init_task_1(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		self.task_1_data.init(planetary_system, active_groups);
	}

	fn init_task_2(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		self.task_2_data.init(planetary_system, active_groups);
	}

	fn init_task_2_rotated(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		self.task_2_rotated_data.init(planetary_system, active_groups);
	}
}

pub struct Task1Data {
	pub plot_width: f64,
	pub points: Vec<(f64, f64, Color32, String, usize)>,
	pub r_squared: f64,
	pub slope: f64,
}

impl Task1Data {
	pub fn new() -> Self {
		Self {
			plot_width: 1.0,
			points: Vec::new(),
			r_squared: 1.0,
			slope: 1.0,
		}
	}

	fn init(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		let mut points_all = Vec::new();
		for object in &planetary_system.objects {
			points_all.push((
				object.distance_au.powf(1.5),
				object.period_years,
				object.colour,
				object.name.clone(),
				*active_groups.get(&object.group).unwrap_or(&true),
			));
		}
		points_all.sort_by(|a, b| a.0.total_cmp(&b.0));
		let mut points = Vec::new();
		let mut vals = Vec::new();
		for (i, (distance, period, colour, name, active)) in points_all.iter().enumerate() {
			if !*active {
				continue;
			}
			let (&distance, &period, &colour) = (distance, period, colour);
			points.push((distance, period, colour, name.clone(), i));
			vals.push((distance, period));
		}
		self.points = points;
		let mut sum_y = 0.0;
		let mut sum_xy = 0.0;
		let mut sum_xx = 0.0;
		let vals_count = vals.len();
		for &(x, y) in &vals {
			sum_y += y;
			sum_xy += x * y;
			sum_xx += x.powi(2);
		}
		let slope = sum_xy / sum_xx;

		let mean_y = sum_y / (vals_count as f64);
		let mut ss_res = 0.0;
		let mut ss_tot = 0.0;
		for (x, y) in vals {
			ss_res += (y - slope * x).powi(2);
			ss_tot += (slope * x - mean_y).powi(2);
		}
		let r_squared = 1.0 - ss_res / ss_tot;
		self.slope = slope;
		self.r_squared = r_squared;
	}
}

pub struct Task2Data {
	pub plot_width: f64,
	/// [([(x, y)], colour, index, name, add_marker)]
	pub points: Vec<(Vec<[f64; 2]>, Color32, usize, String, bool)>,
}

impl Task2Data {
	pub fn new() -> Self {
		Self { plot_width: 1.0, points: Vec::new() }
	}

	fn init(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		let mut points_all = Vec::new();
		for object in &planetary_system.objects {
			points_all.push((
				object.distance_au,
				object.eccentricity,
				object.colour,
				object.name.clone(),
				*active_groups.get(&object.group).unwrap_or(&true),
			));
		}
		points_all.sort_by(|a, b| a.0.total_cmp(&b.0));
		let mut points = Vec::new();
		for (index, (distance, eccentricity, colour, name, active)) in points_all.iter().enumerate() {
			if !*active {
				continue;
			}
			let (&distance, &eccentricity, &colour) = (distance, eccentricity, colour);
			let points_object = (0..=constants::TASK_2_STEPS)
				.map(|i| {
					let theta = eframe::emath::remap(i as f64, 0.0..=(constants::TASK_2_STEPS as f64), 0.0..=TAU);
					let r = (distance * (1.0 - eccentricity.powi(2))) / (1.0 - eccentricity * theta.cos());
					let x = r * theta.cos();
					let y = r * theta.sin();
					[x, y]
				})
				.collect::<Vec<[f64; 2]>>();
			points.push((points_object, colour, index, name.clone(), distance == 0.0));
		}
		self.points = points;
	}
}

pub struct Task2RotatedData {
	pub plot_width: f64,
	/// [([(x, y)], colour, index, name, add_marker)]
	pub points: Vec<(Vec<[f64; 2]>, Color32, usize, String, bool)>,
}

impl Task2RotatedData {
	pub fn new() -> Self {
		Self { plot_width: 1.0, points: Vec::new() }
	}

	fn init(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		let mut points_all = Vec::new();
		for object in &planetary_system.objects {
			points_all.push((
				object.distance_au,
				object.eccentricity,
				object.longitude_of_perihelion,
				object.colour,
				object.name.clone(),
				*active_groups.get(&object.group).unwrap_or(&true),
			));
		}
		points_all.sort_by(|a, b| a.0.total_cmp(&b.0));
		let mut points = Vec::new();
		for (index, (distance, eccentricity, longitude_of_perihelion, colour, name, active)) in points_all.iter().enumerate() {
			if !*active {
				continue;
			}
			let (&distance, &eccentricity, &longitude_of_perihelion, &colour) = (distance, eccentricity, longitude_of_perihelion, colour);
			let longitude_of_perihelion = longitude_of_perihelion * PI / 180.0;
			let points_object = (0..=constants::TASK_2_STEPS)
				.map(|i| {
					let theta = eframe::emath::remap(i as f64, 0.0..=(constants::TASK_2_STEPS as f64), 0.0..=TAU);
					let r = (distance * (1.0 - eccentricity.powi(2))) / (1.0 - eccentricity * (PI + theta - longitude_of_perihelion).cos());
					let x = r * theta.cos();
					let y = r * theta.sin();
					[x, y]
				})
				.collect::<Vec<[f64; 2]>>();
			points.push((points_object, colour, index, name.clone(), distance == 0.0));
		}
		self.points = points;
	}
}
