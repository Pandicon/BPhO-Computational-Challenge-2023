use std::{collections::HashMap, f64::consts::TAU};

use eframe::{egui, epaint::Color32};

use crate::{constants, structs};

pub struct Task3Data {
	pub plot_width: f64,
	/// [([(x, y)], colour, index, name)]
	pub points: Vec<(Vec<[f64; 2]>, Color32, usize, String)>,
	/// [([(x, y)], colour)]
	pub markers: Vec<([f64; 2], Color32)>,
	pub speed: f64,
	pub time: f64,
}

impl Task3Data {
	pub fn new() -> Self {
		Self {
			plot_width: 1.0,
			points: Vec::new(),
			markers: Vec::new(),
			speed: 1.0,
			time: 0.0,
		}
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
			points.push((points_object, colour, index, name.clone()));
		}
		self.points = points;
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
				object.colour,
				*active_groups.get(&object.group).unwrap_or(&true),
			));
		}
		points_all.sort_by(|a, b| a.0.total_cmp(&b.0));
		let mut markers = Vec::new();
		for &(distance, period, eccentricity, colour, active) in &points_all {
			if !active {
				continue;
			}
			let theta = TAU * if period != 0.0 { (self.time % period) / period } else { 0.0 };
			markers.push((pos(distance, eccentricity, theta), colour));
		}
		self.markers = markers;
	}
}

fn pos(distance: f64, eccentricity: f64, theta: f64) -> [f64; 2] {
	let r = (distance * (1.0 - eccentricity.powi(2))) / (1.0 - eccentricity * theta.cos());
	let x = r * theta.cos();
	let y = r * theta.sin();
	[x, y]
}
