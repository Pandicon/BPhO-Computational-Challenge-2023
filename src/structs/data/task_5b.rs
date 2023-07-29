use std::{
	collections::HashMap,
	f64::consts::{PI, TAU},
};

use eframe::{egui, epaint::Color32};

use crate::{constants, structs};

const D_THETA: f64 = 0.001;

pub struct Task5BData {
	pub plot_width: f64,
	/// [([(x, y, z)], colour, stoke only, index, name)]
	pub markers: Vec<([f64; 3], Color32, bool, usize, String)>,
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
	pub time_theta: Vec<Vec<[f64; 2]>>,
}

impl Task5BData {
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
			time_theta: Vec::new(),
		}
	}

	pub fn init(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		let mut points_all = Vec::new();
		for object in &planetary_system.objects {
			points_all.push((
				object.distance_au,
				object.period_years,
				object.eccentricity,
				object.inclination,
				object.colour,
				*active_groups.get(&object.group).unwrap_or(&true),
			));
		}
		points_all.sort_by(|a, b| a.0.total_cmp(&b.0));
		let mut points = Vec::new();
		let mut time_vs_theta = Vec::new();
		for &(distance, period, eccentricity, inclination, colour, active) in &points_all {
			if !active {
				continue;
			}
			if distance == 0.0 {
				time_vs_theta.push(vec![[0.0, 0.0]]);
				continue;
			}
			let inclination = inclination * PI / 180.0;
			let points_object = (0..=constants::TASK_5B_STEPS)
				.map(|i| {
					let theta = eframe::emath::remap(i as f64, 0.0..=(constants::TASK_5B_STEPS as f64), 0.0..=TAU);
					pos(distance, eccentricity, inclination, theta)
				})
				.collect::<Vec<[f64; 3]>>();
			points.push((points_object, colour));

			let mut theta = 0.0;
			let mut t_integrand = 0.0;
			let mut i = 0;
			let mut time_vs_theta_this_object = Vec::new();
			while theta < TAU {
				let val = 1.0 / (1.0 - eccentricity * theta.cos()).powi(2);
				t_integrand += 1.0 / 3.0 * D_THETA * val;
				if i % 10 == 0 {
					time_vs_theta_this_object.push([period * (1.0 - eccentricity.powi(2)).powf(1.5) / TAU * t_integrand, theta]);
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

			time_vs_theta_this_object.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
			time_vs_theta.push(time_vs_theta_this_object);
		}
		self.points = points.clone();
		self.time_theta = time_vs_theta;
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
			markers.push((pos(distance, eccentricity, inclination, theta), colour, true, index, name.clone()));
		}
		for (index, (distance, period, eccentricity, inclination, colour, name, active)) in points_all.iter().enumerate() {
			if !*active {
				continue;
			}
			let (&distance, &period, &eccentricity, &inclination, &colour) = (distance, period, eccentricity, inclination, colour);
			let inclination = inclination * PI / 180.0;
			if period == 0.0 {
				markers.push((pos(distance, eccentricity, inclination, 0.0), colour, false, index, name.clone()));
			} else if let Some(theta) = self.angle_from_time(index, self.time % period) {
				markers.push((pos(distance, eccentricity, inclination, theta), colour, false, index, name.clone()));
			}
		}
		self.markers = markers;
	}

	fn angle_from_time(&self, index: usize, time: f64) -> Option<f64> {
		let time_angles = &self.time_theta[index];
		if time_angles.is_empty() {
			return None;
		}
		let times = time_angles.iter().map(|[time, _angle]| *time).collect::<Vec<f64>>();
		if let Some(index) = Self::floor_in_array(&times, 0, times.len() - 1, time) {
			let ([t_0, th_0], [t_1, th_1]) = (time_angles[index], time_angles[(index + 1) % time_angles.len()]);
			let theta = th_0 + (th_1 - th_0) * (time - t_0) / (t_1 - t_0);
			return Some(theta);
		}
		None
	}

	/// https://stackoverflow.com/questions/50692011/find-the-first-element-in-a-sorted-array-that-is-smaller-than-the-target
	fn floor_in_array(arr: &Vec<f64>, low: usize, high: usize, val: f64) -> Option<usize> {
		if val < arr[0] {
			return Some(arr.len() - 1); // This means it is between the last calculated point and the first one
		}
		if low > high {
			return None;
		}
		if val >= arr[high] {
			return Some(high);
		}
		let mid = (low + high) / 2;
		if arr[mid] == val {
			return Some(mid);
		}
		if mid > 0 && arr[mid - 1] <= val && val < arr[mid] {
			return Some(mid - 1);
		}

		if val < arr[mid] {
			if mid > 0 {
				return Self::floor_in_array(arr, low, mid - 1, val);
			} else {
				println!("rip");
				return None;
			}
		}
		Self::floor_in_array(arr, mid + 1, high, val)
	}
}

fn pos(distance: f64, eccentricity: f64, inclination: f64, theta: f64) -> [f64; 3] {
	let r = (distance * (1.0 - eccentricity.powi(2))) / (1.0 - eccentricity * theta.cos());
	let x = r * theta.cos();
	let y = r * theta.sin();
	[x * inclination.cos(), y, x * inclination.sin()]
}
