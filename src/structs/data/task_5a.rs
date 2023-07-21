use std::{collections::HashMap, f64::consts::TAU};

use eframe::epaint::Color32;

use crate::{constants, structs};

const D_THETA: f64 = 0.001;
const ORBITS_COUNT: f64 = 3.0;

pub struct Task5AData {
	/// [([(t, theta)], colour, index, name, add_marker, dashed)]
	pub points: Vec<(Vec<[f64; 2]>, Color32, usize, String, bool)>,
}

impl Task5AData {
	pub fn new() -> Self {
		Self { points: Vec::new() }
	}

	pub fn init(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		let mut points_all = Vec::new();
		for object in &planetary_system.objects {
			points_all.push((
				object.distance_au,
				object.eccentricity,
				object.period_years,
				object.colour,
				object.name.clone(),
				*active_groups.get(&object.group).unwrap_or(&true),
			));
		}
		points_all.sort_by(|a, b| a.0.total_cmp(&b.0));
		let mut points = Vec::new();
		for (index, (_distance, eccentricity, period, colour, name, active)) in points_all.iter().enumerate() {
			if !*active {
				continue;
			}
			let (&eccentricity, &period, &colour) = (eccentricity, period, colour);
			if period == 0.0 {
				continue;
			}
			let points_object = (0..=constants::TASK_2_STEPS)
				.map(|i| {
					let theta = eframe::emath::remap(i as f64, 0.0..=(constants::TASK_5A_STEPS as f64), 0.0..=(ORBITS_COUNT * TAU));
					let t = theta / TAU * period;
					[t, theta]
				})
				.collect::<Vec<[f64; 2]>>();
			points.push((points_object, colour, index, format!("{} (ε = 0)", name), true));

			let mut points_object = Vec::new();
			let mut theta = 0.0;
			let mut t_integrand = 0.0;
			let mut i = 0;
			while theta < TAU * ORBITS_COUNT {
				let val = 1.0 / (1.0 - eccentricity * theta.cos()).powi(2);
				t_integrand += 1.0 / 3.0 * D_THETA * val;
				if i % 100 == 0 {
					points_object.push([period * (1.0 - eccentricity.powi(2)).powf(1.5) / TAU * t_integrand, theta]);
				}
				theta += D_THETA;
				if i > 0 {
					if (i - 1) % 2 == 0 {
						t_integrand += D_THETA * val; // + 3 * 1/3*h*val
					} else {
						t_integrand += 1.0 / 3.0 * D_THETA * val; // + 1 * 1/3*h*val
					}
				}
				i += 1;
			}
			points.push((points_object, colour, index, format!("{} (ε = {:.3})", name, eccentricity), false));
		}
		self.points = points;
	}
}
