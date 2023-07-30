use std::{
	collections::HashMap,
	f64::consts::{PI, TAU},
};

use eframe::{egui, epaint::Color32};

use crate::structs;

const D_THETA: f64 = 0.001;

pub struct Task7Data {
	pub plot_width: f64,
	/// [([(x, y, z)], colour, index, name)]
	pub markers: Vec<([f64; 3], Color32, usize, String)>,
	/// [([(x, y, z)], colour)]
	pub points: Vec<(Vec<[f64; 3]>, Color32)>,
	pub points_per_orbit: f64,
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
	pub stationary_object_index: usize,
}

impl Task7Data {
	pub fn new() -> Self {
		Self {
			plot_width: 1.0,
			markers: Vec::new(),
			points: Vec::new(),
			points_per_orbit: 1000.0,
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
			stationary_object_index: 0,
		}
	}

	pub fn init(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		let mut points_all = Vec::new();
		for object in &planetary_system.objects {
			points_all.push((object.distance_au, object.period_years, object.eccentricity));
		}
		points_all.sort_by(|a, b| a.0.total_cmp(&b.0));
		let mut time_vs_theta = Vec::new();
		for &(distance, period, eccentricity) in &points_all {
			if distance == 0.0 {
				time_vs_theta.push(vec![[0.0, 0.0]]);
				continue;
			}

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
		self.points = planetary_system.objects.iter().map(|object| (Vec::new(), object.colour)).collect();
		self.time = 0.0;
		self.time_theta = time_vs_theta;
	}

	pub fn step(&mut self, ctx: &egui::Context, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
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
		let mut positions = Vec::new();
		for (index, (distance, period, eccentricity, inclination, colour, name, active)) in points_all.iter().enumerate() {
			if !*active && index != self.stationary_object_index {
				positions.push(None);
				continue;
			}
			let (&distance, &period, &eccentricity, &inclination, &colour) = (distance, period, eccentricity, inclination, colour);
			let inclination = inclination * PI / 180.0;

			if let Some(theta) = self.angle_from_time(index, self.time % period) {
				positions.push(Some((pos(distance, eccentricity, inclination, theta), colour, period, index, name.clone())));
			} else if distance == 0.0 {
				positions.push(Some(([0.0, 0.0, 0.0], colour, period, index, name.clone())));
			} else {
				positions.push(None);
			}
		}
		if let Some((stationary_position, _, _, _, _)) = positions[self.stationary_object_index] {
			let positions = positions
				.iter()
				.map(|pos| {
					if let Some((position, colour, period, index, name)) = pos {
						Some((
							[position[0] - stationary_position[0], position[1] - stationary_position[1], position[2] - stationary_position[2]],
							*colour,
							*period,
							*index,
							name.clone(),
						))
					} else {
						None
					}
				})
				.collect::<Vec<Option<([f64; 3], Color32, f64, usize, String)>>>();
			for (i, pos) in positions.iter().enumerate() {
				if let Some((position, colour, period, index, name)) = pos {
					if self.points_per_orbit < 0.0 || (self.points[i].0.len() as f64) < self.time / period * self.points_per_orbit {
						self.points[i].0.push(*position);
					} else if !self.points[i].0.is_empty() {
						let index = self.points[i].0.len() - 1;
						self.points[i].0[index] = *position;
					}
					markers.push((*position, *colour, *index, name.clone()))
				}
			}
			self.markers = markers;
		}
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
