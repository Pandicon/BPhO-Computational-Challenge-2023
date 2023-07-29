use std::f32::consts::PI;

use eframe::{egui, epaint::Color32};
use nalgebra::{Matrix3, Vector3};

use crate::application::Application;

const LABELS_TOP_MARGIN: f32 = 5.0;
const LABELS_LEFT_MARGIN: f32 = 5.0;
const LABELS_PADDING: f32 = 5.0;
const LABELS_GAP: f32 = 5.0;
const LABELS_CIRCLE_RADIUS: f32 = 5.0;
const LABELS_CIRCLE_STROKE: f32 = 2.0;
const LABELS_CIRCLE_LABEL_GAP: f32 = 7.0;
const LABELS_RECT_STROKE_WIDTH: f32 = 2.0;
const LIGHT_COLOUR: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 255);

impl Application {
	pub fn render_task_7(&mut self, ctx: &egui::Context) {
		egui::CentralPanel::default().show(ctx, |ui| {
			self.data
				.task_7_data
				.step(ctx, &self.planetary_systems[self.chosen_system], &self.active_groups[self.chosen_task.task_index()][self.chosen_system]);
			let zoom = 1.1_f32.powf(self.data.task_7_data.zoom_coefficient);
			let graph_offset_x = self.data.task_7_data.offset_x;
			let graph_offset_y = self.data.task_7_data.offset_y;
			let viewport_rect = ctx.input(|i| i.screen_rect());
			let win_offset_x = (viewport_rect.max.x + viewport_rect.min.x) / 2.0;
			let win_offset_y = (viewport_rect.max.y + viewport_rect.min.y) / 2.0;
			let painter = ui.painter();
			let (rot_x_sin, rot_x_cos) = (self.data.task_7_data.rotate_x * PI / 180.0).sin_cos();
			let (rot_y_sin, rot_y_cos) = (self.data.task_7_data.rotate_y * PI / 180.0).sin_cos();
			let projection_matrix =
				Matrix3::new(1.0, 0.0, 0.0, 0.0, rot_x_cos, rot_x_sin, 0.0, -rot_x_sin, rot_x_cos) * Matrix3::new(rot_y_cos, 0.0, rot_y_sin, 0.0, 1.0, 0.0, -rot_y_sin, 0.0, rot_y_cos);

			let mut labels = Vec::new();

			let mut lines_vertices = Vec::new();
			for (points, colour) in &self.data.task_7_data.points {
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
			lines_vertices.sort_by(|&(a, _), &(b, _)| (a[0][2] + a[1][2]).partial_cmp(&(b[0][2] + b[1][2])).unwrap());
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
			for ([x, y, z], colour, index, name) in &self.data.task_7_data.markers {
				let v = projection_matrix * Vector3::new(*x as f32, *z as f32, *y as f32); // Swapping y and z is needed since in rendering the y-axis is usually pointing upwards
				markers.push(([v.x, v.y, v.z], *colour));
				labels.push((format!("[{}] {}", index, name), *colour));
			}
			markers.sort_by(|(a, ..), (b, ..)| b[2].partial_cmp(&a[2]).unwrap());
			for &([x, y, _z], colour) in &markers {
				painter.circle_filled(egui::Pos2::new(win_offset_x + graph_offset_x + x * zoom, win_offset_y + graph_offset_y + y * zoom), 6.0, colour);
			}

			let label_rect = egui::Label::new(
				egui::RichText::new(format!("T = {:.3} year{}", self.data.task_7_data.time, if self.data.task_7_data.time == 1.0 { "" } else { "s" }))
					.color(LIGHT_COLOUR)
					.size(18.0),
			)
			.layout_in_ui(ui)
			.2
			.rect;
			let heading_label_height = label_rect.max.y - label_rect.min.y;
			let heading_label_width = label_rect.max.x - label_rect.min.x;
			let top = self.data.top_panel_bottom;
			let left = LABELS_LEFT_MARGIN + LABELS_RECT_STROKE_WIDTH;
			ui.put(
				egui::Rect::from_two_pos(egui::pos2(left, top), egui::pos2(left + heading_label_width, top + heading_label_height)),
				egui::Label::new(
					egui::RichText::new(format!("T = {:.3} year{}", self.data.task_7_data.time, if self.data.task_7_data.time == 1.0 { "" } else { "s" }))
						.color(LIGHT_COLOUR)
						.size(18.0),
				),
			);

			let painter = ui.painter();
			labels.sort_by(|(a, ..), (b, ..)| a.cmp(&b));
			painter.rect_filled(
				egui::Rect::from_two_pos(
					egui::pos2(LABELS_LEFT_MARGIN, LABELS_TOP_MARGIN + self.data.top_panel_bottom + heading_label_height),
					egui::pos2(
						LABELS_LEFT_MARGIN + self.data.task_7_data.labels_width,
						LABELS_TOP_MARGIN + self.data.top_panel_bottom + self.data.task_7_data.labels_height + heading_label_height,
					),
				),
				1.0,
				crate::constants::CENTRAL_PANEL_BG,
			);
			painter.rect_stroke(
				egui::Rect::from_two_pos(
					egui::pos2(LABELS_LEFT_MARGIN, LABELS_TOP_MARGIN + self.data.top_panel_bottom + heading_label_height),
					egui::pos2(
						LABELS_LEFT_MARGIN + self.data.task_7_data.labels_width,
						LABELS_TOP_MARGIN + self.data.top_panel_bottom + self.data.task_7_data.labels_height + heading_label_height,
					),
				),
				1.0,
				egui::Stroke::new(LABELS_RECT_STROKE_WIDTH, LIGHT_COLOUR),
			);
			let mut only_labels_height = 0.0;
			let mut max_width = 0.0;
			for (i, (text, colour)) in labels.iter().enumerate() {
				let top = self.data.top_panel_bottom + LABELS_TOP_MARGIN + LABELS_PADDING + (i as f32) * LABELS_GAP + only_labels_height + heading_label_height;
				let left = LABELS_LEFT_MARGIN + LABELS_RECT_STROKE_WIDTH + LABELS_PADDING;

				let label_left = left + LABELS_CIRCLE_RADIUS * 2.0 + LABELS_CIRCLE_LABEL_GAP;
				let label_rect = egui::Label::new(egui::RichText::new(text).color(LIGHT_COLOUR)).layout_in_ui(ui).2.rect;
				let label_height = label_rect.max.y - label_rect.min.y;
				let label_width = label_rect.max.x - label_rect.min.x;
				ui.put(
					egui::Rect::from_two_pos(egui::pos2(label_left, top), egui::pos2(label_left + label_width, top + label_height)),
					egui::Label::new(egui::RichText::new(text).color(LIGHT_COLOUR)),
				);
				only_labels_height += label_height;
				let total_width = label_left + label_width + LABELS_PADDING - LABELS_LEFT_MARGIN;
				if total_width > max_width {
					max_width = total_width;
				}

				let painter = ui.painter();
				let circle_centre_x = left + LABELS_CIRCLE_RADIUS;
				let circle_centre_y = top + label_height / 2.0;
				painter.circle_filled(egui::pos2(circle_centre_x, circle_centre_y), LABELS_CIRCLE_RADIUS, *colour);
			}
			self.data.task_7_data.labels_height = only_labels_height + ((labels.len() - 1) as f32) * LABELS_GAP + 2.0 * LABELS_PADDING;
			self.data.task_7_data.labels_width = max_width;
		});
	}
}
