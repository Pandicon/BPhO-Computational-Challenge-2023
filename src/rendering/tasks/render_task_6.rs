use eframe::{egui, epaint::Color32};

use crate::{application::Application, constants};

const LABELS_TOP_MARGIN: f32 = 5.0;
const LABELS_LEFT_MARGIN: f32 = 5.0;
const LABELS_PADDING: f32 = 5.0;
const LABELS_GAP: f32 = 5.0;
const LABELS_CIRCLE_RADIUS: f32 = 5.0;
const LABELS_CIRCLE_LABEL_GAP: f32 = 7.0;
const LABELS_RECT_STROKE_WIDTH: f32 = 2.0;
const LIGHT_COLOUR: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 255);

impl Application {
	pub fn render_task_6(&mut self, ctx: &egui::Context) {
		egui::CentralPanel::default().show(ctx, |ui| {
			let zoom = 1.1_f32.powf(self.data.task_6_data.zoom_coefficient);
			let graph_offset_x = self.data.task_6_data.offset_x;
			let graph_offset_y = self.data.task_6_data.offset_y;
			let viewport_rect = ctx.input(|i| i.screen_rect());
			let win_offset_x = (viewport_rect.max.x + viewport_rect.min.x) / 2.0;
			let win_offset_y = (viewport_rect.max.y + viewport_rect.min.y) / 2.0;
			let painter = ui.painter();

			let mut labels = Vec::new();

			let mut lines_vertices = Vec::new();
			for (points, colour) in &self.data.task_6_data.orbit_points {
				if points.is_empty() {
					continue;
				}
				let points = points.iter().map(|&[x, y]| [x as f32, y as f32]).collect::<Vec<[f32; 2]>>();
				for i in 0..(points.len() - 1) {
					lines_vertices.push(([points[i], points[i + 1]], colour));
				}
			}
			let mut spirograph_lines_vertices = Vec::new();
			for points in &self.data.task_6_data.spirograph_line_points {
				if points.is_empty() {
					continue;
				}
				let points = points.iter().map(|&[x, y]| [x as f32, y as f32]).collect::<Vec<[f32; 2]>>();
				spirograph_lines_vertices.push(([points[0], points[1]], &constants::SPIROGRAPH_LINES_COLOUR));
			}
			for &([pos_s, pos_n], colour) in &spirograph_lines_vertices {
				painter.line_segment(
					[
						egui::Pos2::new(win_offset_x + graph_offset_x + pos_s[0] * zoom, win_offset_y + graph_offset_y + pos_s[1] * zoom),
						egui::Pos2::new(win_offset_x + graph_offset_x + pos_n[0] * zoom, win_offset_y + graph_offset_y + pos_n[1] * zoom),
					],
					egui::Stroke::new(self.data.task_6_data.line_width, *colour),
				)
			}
			for &([pos_s, pos_n], colour) in &lines_vertices {
				painter.line_segment(
					[
						egui::Pos2::new(win_offset_x + graph_offset_x + pos_s[0] * zoom, win_offset_y + graph_offset_y + pos_s[1] * zoom),
						egui::Pos2::new(win_offset_x + graph_offset_x + pos_n[0] * zoom, win_offset_y + graph_offset_y + pos_n[1] * zoom),
					],
					egui::Stroke::new(3.0, *colour),
				)
			}

			for (colour, index, name) in &self.data.task_6_data.labels {
				labels.push((format!("[{}] {}", index, name), *colour));
			}
			if !labels.is_empty() {
				let painter = ui.painter();
				labels.sort_by(|(a, ..), (b, ..)| a.cmp(b));
				painter.rect_filled(
					egui::Rect::from_two_pos(
						egui::pos2(LABELS_LEFT_MARGIN, LABELS_TOP_MARGIN + self.data.top_panel_bottom),
						egui::pos2(
							LABELS_LEFT_MARGIN + self.data.task_6_data.labels_width,
							LABELS_TOP_MARGIN + self.data.top_panel_bottom + self.data.task_6_data.labels_height,
						),
					),
					1.0,
					crate::constants::CENTRAL_PANEL_BG,
				);
				painter.rect_stroke(
					egui::Rect::from_two_pos(
						egui::pos2(LABELS_LEFT_MARGIN, LABELS_TOP_MARGIN + self.data.top_panel_bottom),
						egui::pos2(
							LABELS_LEFT_MARGIN + self.data.task_6_data.labels_width,
							LABELS_TOP_MARGIN + self.data.top_panel_bottom + self.data.task_6_data.labels_height,
						),
					),
					1.0,
					egui::Stroke::new(LABELS_RECT_STROKE_WIDTH, LIGHT_COLOUR),
				);
				let mut only_labels_height = 0.0;
				let mut max_width = 0.0;
				for (i, (text, colour)) in labels.iter().enumerate() {
					let top = self.data.top_panel_bottom + LABELS_TOP_MARGIN + LABELS_PADDING + (i as f32) * LABELS_GAP + only_labels_height;
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
				self.data.task_6_data.labels_height = only_labels_height + ((labels.len() - 1) as f32) * LABELS_GAP + 2.0 * LABELS_PADDING;
				self.data.task_6_data.labels_width = max_width;
			}
		});
	}
}
