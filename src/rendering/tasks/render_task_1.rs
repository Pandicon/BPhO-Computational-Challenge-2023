use std::{f64::consts::TAU, ops::RangeInclusive};

use eframe::{egui, epaint::Color32};

use crate::application::Application;

const BEST_FIT_LINE_POINTS: usize = 256;
const PLANETS_RADIUS_FRACTION: f64 = 1.0 / 200.0;
const PLANETS_CIRCLE_POINTS: usize = 256;

impl Application {
	pub fn render_task_1(&mut self, ctx: &egui::Context) {
		egui::CentralPanel::default().show(ctx, |ui| {
			let x_fmt = |x: f64, _range: &RangeInclusive<f64>| format!("{:.3} AU", x.powf(2.0 / 3.0));

			let y_fmt = |y: f64, _range: &RangeInclusive<f64>| format!("{:.3} year{}", y, if y == 1.0 { "" } else { "s" });

			let label_fmt = |_s: &str, val: &egui::plot::PlotPoint| format!("{:.3} AU\n{:.3} year{}", val.x.powf(2.0 / 3.0), val.y, if val.y == 1.0 { "" } else { "s" });

			let plot = egui::plot::Plot::new("T-A relationship")
				.data_aspect(1.0)
				.x_axis_formatter(x_fmt)
				.y_axis_formatter(y_fmt)
				.label_formatter(label_fmt)
				.legend(egui::plot::Legend::default());

			let mut object_points = Vec::new();
			let mut object_lines = Vec::new();
			let radius = self.data.task_1_data.plot_width * PLANETS_RADIUS_FRACTION;
			let mut min_x = f64::INFINITY;
			let mut max_x = f64::NEG_INFINITY;
			for (x, y, colour, name) in &self.data.task_1_data.points {
				let (&x, &y, &colour) = (x, y, colour);
				object_points.push(egui::plot::Points::new(vec![[x, y]]).color(colour).highlight(true));
				let circle_points: egui::plot::PlotPoints = (0..=PLANETS_CIRCLE_POINTS)
					.map(|i| {
						let t = eframe::emath::remap(i as f64, 0.0..=(PLANETS_CIRCLE_POINTS as f64), 0.0..=TAU);
						let r = radius;
						[r * t.cos() + x, r * t.sin() + y]
					})
					.collect();
				object_lines.push(egui::plot::Line::new(circle_points).color(colour).highlight(true).name(name.to_owned()));
				object_lines.push(egui::plot::Line::new(egui::plot::PlotPoints::new(vec![[x - radius, y], [x + radius, y]])).color(colour).highlight(true));
				object_lines.push(egui::plot::Line::new(egui::plot::PlotPoints::new(vec![[x, y - radius], [x, y + radius]])).color(colour).highlight(true));
				if x > max_x {
					max_x = x;
				}
				if x < min_x {
					min_x = x;
				}
			}
			let best_fit_points: egui::plot::PlotPoints = (0..=BEST_FIT_LINE_POINTS)
				.map(|i| {
					let x = eframe::emath::remap(i as f64, 0.0..=(BEST_FIT_LINE_POINTS as f64), min_x..=max_x);
					[x, self.data.task_1_data.slope * x]
				})
				.collect();
			let best_fit_line = egui::plot::Line::new(best_fit_points).color(Color32::RED).name(format!("y = {}x", self.data.task_1_data.slope));
			let plot_bounds = plot
				.show(ui, |plot_ui| {
					plot_ui.line(best_fit_line);
					for line in object_lines {
						plot_ui.line(line);
					}
					for point in object_points {
						plot_ui.points(point);
					}

					plot_ui.plot_bounds()
				})
				.inner;
			let plot_width = plot_bounds.max()[0] - plot_bounds.min()[0];
			self.data.task_1_data.plot_width = plot_width;
		});
	}
}
