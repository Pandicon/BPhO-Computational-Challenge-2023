use std::{f64::consts::TAU, ops::RangeInclusive};

use eframe::{egui, epaint::Color32};

use crate::application::Application;

const MARKERS_RADIUS_FRACTION: f64 = 1.0 / 200.0;
const MARKERS_CIRCLE_POINTS: usize = 256;

const LABELS_LEFT_MARGIN: f32 = 5.0;
const LABELS_RECT_STROKE_WIDTH: f32 = 2.0;
const LIGHT_COLOUR: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 255);

impl Application {
	pub fn render_task_3(&mut self, ctx: &egui::Context) {
		self.data
			.task_3_data
			.move_markers(ctx, &self.planetary_systems[self.chosen_system], &self.active_groups[self.chosen_task.task_index()][self.chosen_system]);
		egui::CentralPanel::default().show(ctx, |ui| {
			let axis_fmt = |val: f64, _range: &RangeInclusive<f64>| format!("{:.3} AU", val);

			let label_fmt = |_s: &str, val: &egui::plot::PlotPoint| format!("x: {:.3} AU\ny: {:.3} AU\ndistance: {:.3} AU", val.x, val.y, (val.x.powi(2) + val.y.powi(2)).sqrt());

			let plot = egui::plot::Plot::new("Orbits of planets")
				.data_aspect(1.0)
				.x_axis_formatter(axis_fmt)
				.y_axis_formatter(axis_fmt)
				.label_formatter(label_fmt)
				.legend(egui::plot::Legend::default());

			let radius = self.data.task_3_data.plot_width * MARKERS_RADIUS_FRACTION;
			let mut marker_lines = Vec::new();
			let mut orbits = Vec::new();
			for (points, colour, index, name) in &self.data.task_3_data.points {
				let (&index, &colour) = (index, colour);
				let mut orbit_points = Vec::new();
				for &[x, y] in points {
					orbit_points.push([x, y]);
				}
				orbits.push(egui::plot::Line::new(orbit_points).color(colour).highlight(true).name(format!("[{}] {}", index, name.to_owned())));
				for &(pos, colour) in &self.data.task_3_data.markers {
					let circle_points: egui::plot::PlotPoints = (0..=MARKERS_CIRCLE_POINTS)
						.map(|i| {
							let t = eframe::emath::remap(i as f64, 0.0..=(MARKERS_CIRCLE_POINTS as f64), 0.0..=TAU);
							let r = radius;
							[r * t.cos() + pos[0], r * t.sin() + pos[1]]
						})
						.collect();
					marker_lines.push(egui::plot::Line::new(circle_points).color(colour).highlight(true));
					marker_lines.push(
						egui::plot::Line::new(egui::plot::PlotPoints::new(vec![[pos[0] - radius, pos[1]], [pos[0] + radius, pos[1]]]))
							.color(colour)
							.highlight(true),
					);
					marker_lines.push(
						egui::plot::Line::new(egui::plot::PlotPoints::new(vec![[pos[0], pos[1] - radius], [pos[0], pos[1] + radius]]))
							.color(colour)
							.highlight(true),
					);
				}
			}
			let plot_bounds = plot
				.show(ui, |plot_ui| {
					for line in orbits {
						plot_ui.line(line);
					}
					for line in marker_lines {
						plot_ui.line(line);
					}

					plot_ui.plot_bounds()
				})
				.inner;

			let plot_width = plot_bounds.max()[0] - plot_bounds.min()[0];
			self.data.task_3_data.plot_width = plot_width;

			let label_rect = egui::Label::new(
				egui::RichText::new(format!("T = {:.3} year{}", self.data.task_3_data.time, if self.data.task_3_data.time == 1.0 { "" } else { "s" }))
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
					egui::RichText::new(format!("T = {:.3} year{}", self.data.task_3_data.time, if self.data.task_3_data.time == 1.0 { "" } else { "s" }))
						.color(LIGHT_COLOUR)
						.size(18.0),
				),
			);
		});
	}
}
