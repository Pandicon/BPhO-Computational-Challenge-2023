use std::{f64::consts::TAU, ops::RangeInclusive};

use eframe::egui;

use crate::application::Application;

const MARKERS_RADIUS_FRACTION: f64 = 1.0 / 200.0;
const MARKERS_CIRCLE_POINTS: usize = 256;

impl Application {
	pub fn render_task_2(&mut self, ctx: &egui::Context) {
		egui::CentralPanel::default().show(ctx, |ui| {
			let axis_fmt = |val: f64, _range: &RangeInclusive<f64>| format!("{:.3} AU", val);

			let label_fmt = |_s: &str, val: &egui::plot::PlotPoint| format!("{:.3} AU\n{:.3} AU", val.x, val.y);

			let plot = egui::plot::Plot::new("Orbits of planets")
				.data_aspect(1.0)
				.x_axis_formatter(axis_fmt)
				.y_axis_formatter(axis_fmt)
				.label_formatter(label_fmt)
				.legend(egui::plot::Legend::default());

			let radius = self.data.task_2_data.plot_width * MARKERS_RADIUS_FRACTION;
			let mut marker_lines = Vec::new();
			let mut orbits = Vec::new();
			for (points, colour, index, name, add_marker) in &self.data.task_2_data.points {
				let (&index, &colour, &add_marker) = (index, colour, add_marker);
				let mut orbit_points = Vec::new();
				for &[x, y] in points {
					orbit_points.push([x, y]);
				}
				orbits.push(egui::plot::Line::new(orbit_points).color(colour).highlight(true).name(format!("[{}] {}", index, name.to_owned())));
				if add_marker {
					let circle_points: egui::plot::PlotPoints = (0..=MARKERS_CIRCLE_POINTS)
						.map(|i| {
							let t = eframe::emath::remap(i as f64, 0.0..=(MARKERS_CIRCLE_POINTS as f64), 0.0..=TAU);
							let r = radius;
							[r * t.cos() + points[0][0], r * t.sin() + points[0][1]]
						})
						.collect();
					marker_lines.push(egui::plot::Line::new(circle_points).color(colour).highlight(true));
					marker_lines.push(
						egui::plot::Line::new(egui::plot::PlotPoints::new(vec![[points[0][0] - radius, points[0][1]], [points[0][0] + radius, points[0][1]]]))
							.color(colour)
							.highlight(true),
					);
					marker_lines.push(
						egui::plot::Line::new(egui::plot::PlotPoints::new(vec![[points[0][0], points[0][1] - radius], [points[0][0], points[0][1] + radius]]))
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
			self.data.task_2_data.plot_width = plot_width;
		});
	}
}
