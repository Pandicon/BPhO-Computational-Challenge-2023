use std::f32::consts::PI;

use eframe::egui;
use nalgebra::{Matrix3, Vector3};

use crate::application::Application;

impl Application {
	pub fn render_task_4(&mut self, ctx: &egui::Context) {
		egui::CentralPanel::default().show(ctx, |ui| {
			self.data
				.task_4_data
				.move_markers(ctx, &self.planetary_systems[self.chosen_system], &self.active_groups[self.chosen_task.task_index()][self.chosen_system]);
			let zoom = 1.1_f32.powf(self.data.task_4_data.zoom_coefficient);
			let graph_offset_x = self.data.task_4_data.offset_x;
			let graph_offset_y = self.data.task_4_data.offset_y;
			let viewport_rect = ctx.input(|i| i.screen_rect());
			let win_offset_x = (viewport_rect.max.x + viewport_rect.min.x) / 2.0;
			let win_offset_y = (viewport_rect.max.y + viewport_rect.min.y) / 2.0;
			let painter = ui.painter();
			let (rot_x_sin, rot_x_cos) = (self.data.task_4_data.rotate_x * PI / 180.0).sin_cos();
			let (rot_y_sin, rot_y_cos) = (self.data.task_4_data.rotate_y * PI / 180.0).sin_cos();
			let projection_matrix =
				Matrix3::new(1.0, 0.0, 0.0, 0.0, rot_x_cos, rot_x_sin, 0.0, -rot_x_sin, rot_x_cos) * Matrix3::new(rot_y_cos, 0.0, rot_y_sin, 0.0, 1.0, 0.0, -rot_y_sin, 0.0, rot_y_cos);

			let mut lines_vertices = Vec::new();
			for (points, colour, index, name) in &self.data.task_4_data.points {
				if points.is_empty() {
					continue;
				}
				let points = points
					.iter()
					.map(|&[x, y, z]| {
						let v = projection_matrix * Vector3::new(x as f32, z as f32, y as f32); // Swapping y and z is needed since in rendering the y-axis is usually pointing upwards
						[v.x, v.y, v.z]
					})
					.collect::<Vec<[f32; 3]>>();
				for i in 0..(points.len() - 1) {
					lines_vertices.push(([points[i], points[i + 1]], colour));
				}
			}
			lines_vertices.sort_by(|&(a, _), &(b, _)| (b[0][2] + b[1][2]).partial_cmp(&(a[0][2] + a[1][2])).unwrap());
			for &([pos_s, pos_n], colour) in &lines_vertices {
				painter.line_segment(
					[
						egui::Pos2::new(win_offset_x + graph_offset_x + pos_s[0] * zoom, win_offset_y + graph_offset_y + pos_s[1] * zoom),
						egui::Pos2::new(win_offset_x + graph_offset_x + pos_n[0] * zoom, win_offset_y + graph_offset_y + pos_n[1] * zoom),
					],
					egui::Stroke::new(3.0, *colour),
				)
			}

			let mut markers = Vec::new();
			for &([x, y, z], colour) in &self.data.task_4_data.markers {
				let v = projection_matrix * Vector3::new(x as f32, z as f32, y as f32); // Swapping y and z is needed since in rendering the y-axis is usually pointing upwards
				markers.push(([v.x, v.y, v.z], colour));
			}
			markers.sort_by(|(a, _), (b, _)| b[2].partial_cmp(&a[2]).unwrap());
			for &([x, y, _z], colour) in &markers {
				painter.circle_filled(egui::Pos2::new(win_offset_x + graph_offset_x + x * zoom, win_offset_y + graph_offset_y + y * zoom), 6.0, colour);
			}
		});
	}
}
