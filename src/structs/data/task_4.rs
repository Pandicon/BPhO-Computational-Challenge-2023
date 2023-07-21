use std::{
	collections::HashMap,
	f64::consts::{PI, TAU},
};

use eframe::{egui, epaint::Color32};

use crate::{constants, structs};

pub struct Task4Data {
	pub plot_width: f64,
	/// [([(x, y, z)], colour, index, name)]
	pub markers: Vec<([f64; 3], Color32, usize, String)>,
	/// [([(x, y, z)], colour)]
	pub points: Vec<(Vec<[f64; 3]>, Color32)>,
	pub time: f64,
	pub speed: f64,
	pub offset_x: f32,
	pub offset_y: f32,
	pub rotate_x: f32,
	pub rotate_y: f32,
	pub zoom_coefficient: f32,
	pub labels_height: f32,
	pub labels_width: f32,
}

impl Task4Data {
	pub fn new() -> Self {
		Self {
			plot_width: 1.0,
			markers: Vec::new(),
			points: Vec::new(),
			time: 0.0,
			speed: 1.0,
			offset_x: 0.0,
			offset_y: 0.0,
			rotate_x: 0.0,
			rotate_y: 0.0,
			zoom_coefficient: 30.0,
			labels_height: 100.0,
			labels_width: 100.0,
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
				*active_groups.get(&object.group).unwrap_or(&true),
			));
		}
		points_all.sort_by(|a, b| a.0.total_cmp(&b.0));
		let mut points = Vec::new();
		for &(distance, eccentricity, inclination, colour, active) in &points_all {
			if !active {
				continue;
			}
			if distance == 0.0 {
				continue;
			}
			let inclination = inclination * PI / 180.0;
			let points_object = (0..=constants::TASK_4_STEPS)
				.map(|i| {
					let theta = eframe::emath::remap(i as f64, 0.0..=(constants::TASK_4_STEPS as f64), 0.0..=TAU);
					pos(distance, eccentricity, inclination, theta)
				})
				.collect::<Vec<[f64; 3]>>();
			points.push((points_object, colour));
		}
		self.points = points.clone();
	}

	pub fn move_markers(&mut self, ctx: &egui::Context, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		let dt = ctx.input(|i| i.stable_dt) as f64;
		self.time += dt * self.speed;
		let mut points_all = Vec::new();
		for object in &planetary_system.objects {
			points_all.push((
				object.distance_au,
				object.period_years,
				object.eccentricity,
				object.inclination,
				object.colour,
				object.name.clone(),
				*active_groups.get(&object.group).unwrap_or(&true),
			));
		}
		points_all.sort_by(|a, b| a.0.total_cmp(&b.0));
		let mut markers = Vec::new();
		for (index, (distance, period, eccentricity, inclination, colour, name, active)) in points_all.iter().enumerate() {
			if !*active {
				continue;
			}
			let (&distance, &period, &eccentricity, &inclination, &colour) = (distance, period, eccentricity, inclination, colour);
			let inclination = inclination * PI / 180.0;
			let theta = TAU * if period != 0.0 { (self.time % period) / period } else { 0.0 };
			markers.push((pos(distance, eccentricity, inclination, theta), colour, index, name.clone()));
		}
		self.markers = markers;
	}
}

fn pos(distance: f64, eccentricity: f64, inclination: f64, theta: f64) -> [f64; 3] {
	let r = (distance * (1.0 - eccentricity.powi(2))) / (1.0 - eccentricity * theta.cos());
	let x = r * theta.cos();
	let y = r * theta.sin();
	[x * inclination.cos(), y, x * inclination.sin()]
}
