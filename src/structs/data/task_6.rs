use std::{collections::HashMap, f64::consts::TAU};

use eframe::epaint::Color32;

use crate::{constants, structs};

const D_THETA: f64 = 0.001;

pub struct Task6Data {
	pub plot_width: f64,
	/// [(colour, index, name)]
	pub labels: Vec<(Color32, usize, String)>,
	/// [([[x, y]], colour)]
	pub orbit_points: Vec<(Vec<[f64; 2]>, Color32)>,
	/// [[[x, y]; 2]]
	pub spirograph_line_points: Vec<[[f64; 2]; 2]>,
	pub dt: f64,
	pub line_width: f32,
	pub number_of_periods: f64,
	pub offset_x: f32,
	pub offset_y: f32,
	pub zoom_coefficient: f32,
	pub labels_height: f32,
	pub labels_width: f32,
	pub time_theta: Vec<Vec<[f64; 2]>>,
	pub chosen_objects: Vec<usize>,
	pub last_valid_pair: [usize; 2],
	pub screen_height: f64,
	pub screen_width: f64,
}

impl Task6Data {
	pub fn new() -> Self {
		Self {
			plot_width: 1.0,
			labels: Vec::new(),
			orbit_points: Vec::new(),
			spirograph_line_points: Vec::new(),
			dt: 0.01,
			line_width: 0.2,
			number_of_periods: 10.0,
			offset_x: 0.0,
			offset_y: 0.0,
			zoom_coefficient: 30.0,
			labels_height: 100.0,
			labels_width: 100.0,
			time_theta: Vec::new(),
			chosen_objects: Vec::new(),
			last_valid_pair: [0, 0],
			screen_height: 0.0,
			screen_width: 0.0,
		}
	}

	pub fn init(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
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
		let mut points = Vec::new();
		let mut time_vs_theta = Vec::new();
		let mut min_x = f64::INFINITY;
		let mut min_y = f64::INFINITY;
		let mut max_x = f64::NEG_INFINITY;
		let mut max_y = f64::NEG_INFINITY;
		for (i, &(distance, period, eccentricity, colour, active)) in points_all.iter().enumerate() {
			if !active {
				continue;
			}
			if distance == 0.0 {
				time_vs_theta.push(vec![[0.0, 0.0]]);
				continue;
			}
			if self.chosen_objects.len() > 1 && self.chosen_objects.contains(&i) {
				let points_object = (0..=constants::TASK_6_STEPS)
					.map(|i| {
						let theta = eframe::emath::remap(i as f64, 0.0..=(constants::TASK_6_STEPS as f64), 0.0..=TAU);
						let pos = pos(distance, eccentricity, theta);
						if pos[0] < min_x {
							min_x = pos[0];
						}
						if pos[0] > max_x {
							max_x = pos[0];
						}
						if pos[1] < min_y {
							min_y = pos[1];
						}
						if pos[1] > max_y {
							max_y = pos[1];
						}
						pos
					})
					.collect::<Vec<[f64; 2]>>();
				points.push((points_object, colour));
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
		if self.chosen_objects.len() > 1 && self.last_valid_pair != [self.chosen_objects[0], self.chosen_objects[1]] {
			self.last_valid_pair = [self.chosen_objects[0], self.chosen_objects[1]];
			self.orbit_points = points;
			self.offset_x = 0.0;
			self.offset_y = 0.0;
			let x_av = self.screen_width / 2.0 * 14.0 / 15.0;
			let y_av = self.screen_height / 2.0 * 14.0 / 15.0;
			let x_req = min_x.abs().max(max_x);
			let y_req = min_y.abs().max(max_y);
			let req_ratio = (x_av / x_req).min(y_av / y_req);
			self.zoom_coefficient = req_ratio.log(1.1) as f32;
			let outer_i = if self.chosen_objects[0] < self.chosen_objects[1] {
				self.chosen_objects[1]
			} else {
				self.chosen_objects[0]
			};
			self.dt = planetary_system.objects[outer_i].period_years * self.number_of_periods / 1000.0;
		}
		self.time_theta = time_vs_theta;

		self.calculate_spirograph(planetary_system);
	}

	pub fn calculate_spirograph(&mut self, planetary_system: &structs::PlanetarySystem) {
		let chosen_objects = &self.chosen_objects;
		if chosen_objects.len() < 2 {
			return;
		}
		let (i_1, i_2) = if chosen_objects[0] < chosen_objects[1] {
			(chosen_objects[0], chosen_objects[1])
		} else {
			(chosen_objects[1], chosen_objects[0])
		};
		self.labels = vec![
			(planetary_system.objects[i_1].colour, i_1, planetary_system.objects[i_1].name.clone()),
			(planetary_system.objects[i_2].colour, i_2, planetary_system.objects[i_2].name.clone()),
		];
		let final_t = planetary_system.objects[i_2].period_years * self.number_of_periods;
		let mut t = 0.0;
		let mut spirograph_lines_points = Vec::new();
		while t < final_t {
			let angle_1 = self.angle_from_time(i_1, t);
			let angle_2 = self.angle_from_time(i_2, t);

			if angle_1.is_none() || angle_2.is_none() {
				continue;
			}
			let (angle_1, angle_2) = (angle_1.unwrap(), angle_2.unwrap());
			let pos_1 = pos(planetary_system.objects[i_1].distance_au, planetary_system.objects[i_1].eccentricity, angle_1);
			let pos_2 = pos(planetary_system.objects[i_2].distance_au, planetary_system.objects[i_2].eccentricity, angle_2);
			spirograph_lines_points.push([pos_1, pos_2]);

			t += self.dt;
		}
		self.spirograph_line_points = spirograph_lines_points;
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

fn pos(distance: f64, eccentricity: f64, theta: f64) -> [f64; 2] {
	let r = (distance * (1.0 - eccentricity.powi(2))) / (1.0 - eccentricity * theta.cos());
	let x = r * theta.cos();
	let y = r * theta.sin();
	[x, y]
}
