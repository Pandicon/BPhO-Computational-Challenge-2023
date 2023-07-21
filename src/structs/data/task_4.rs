use std::{
	collections::HashMap,
	f64::consts::{PI, TAU},
};

use eframe::epaint::Color32;

use crate::{constants, structs};

pub struct Task4Data {
	pub plot_width: f64,
	/// [([(x, y, z)], colour, index, name)]
	pub points: Vec<(Vec<[f64; 3]>, Color32, usize, String)>,
	pub offset_x: f32,
	pub offset_y: f32,
	pub rotate_x: f32,
	pub rotate_y: f32,
	pub zoom_coefficient: f32,
}

impl Task4Data {
	pub fn new() -> Self {
		Self {
			plot_width: 1.0,
			points: Vec::new(),
			offset_x: 0.0,
			offset_y: 0.0,
			rotate_x: 0.0,
			rotate_y: 0.0,
			zoom_coefficient: 30.0,
		}
	}

	pub fn init(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		let mut points_all = Vec::new();
		for object in &planetary_system.objects {
			points_all.push((
				object.distance_au,
				object.eccentricity,
				object.inclination,
				object.colour,
				object.name.clone(),
				*active_groups.get(&object.group).unwrap_or(&true),
			));
		}
		points_all.sort_by(|a, b| a.0.total_cmp(&b.0));
		let mut points = Vec::new();
		for (index, (distance, eccentricity, inclination, colour, name, active)) in points_all.iter().enumerate() {
			if !*active {
				continue;
			}
			let (&distance, &eccentricity, &colour) = (distance, eccentricity, colour);
			if distance == 0.0 {
				continue;
			}
			let inclination = *inclination * PI / 180.0;
			let points_object = (0..=constants::TASK_4_STEPS)
				.map(|i| {
					let theta = eframe::emath::remap(i as f64, 0.0..=(constants::TASK_4_STEPS as f64), 0.0..=TAU);
					let r = (distance * (1.0 - eccentricity.powi(2))) / (1.0 - eccentricity * theta.cos());
					let x = r * theta.cos();
					let y = r * theta.sin();
					[x * inclination.cos(), y, x * inclination.sin()]
				})
				.collect::<Vec<[f64; 3]>>();
			points.push((points_object, colour, index, name.clone()));
		}
		self.points = points.clone();
	}
}
