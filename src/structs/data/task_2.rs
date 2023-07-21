use std::{collections::HashMap, f64::consts::TAU};

use eframe::epaint::Color32;

use crate::{constants, structs};

pub struct Task2Data {
	pub plot_width: f64,
	/// [([(x, y)], colour, index, name, add_marker)]
	pub points: Vec<(Vec<[f64; 2]>, Color32, usize, String, bool)>,
}

impl Task2Data {
	pub fn new() -> Self {
		Self { plot_width: 1.0, points: Vec::new() }
	}

	pub fn init(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
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
