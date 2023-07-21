use std::collections::HashMap;

use eframe::epaint::Color32;

use crate::structs;

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

	pub fn init(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
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
