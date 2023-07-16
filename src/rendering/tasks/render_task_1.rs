use std::ops::RangeInclusive;

use eframe::egui;

use crate::application::Application;

impl Application {
	pub fn render_task_1(&self, ctx: &egui::Context) {
		egui::CentralPanel::default().show(ctx, |ui| {
			let x_fmt = |x: f64, _range: &RangeInclusive<f64>| format!("{:.3} AU", x.powf(2.0 / 3.0));

			let y_fmt = |y: f64, _range: &RangeInclusive<f64>| format!("{:.3} year{}", y, if y == 1.0 { "" } else { "s" });

			let label_fmt = |_s: &str, val: &egui::plot::PlotPoint| format!("{:.3} AU\n{:.3} year{}", val.x.powf(2.0 / 3.0), val.y, if val.y == 1.0 { "" } else { "s" });

			let plot = egui::plot::Plot::new("T-A relationship")
				.data_aspect(1.0)
				.x_axis_formatter(x_fmt)
				.y_axis_formatter(y_fmt)
				.label_formatter(label_fmt);

			let mut points = Vec::new();
			for &(x, y, colour) in &self.data.task_1_data.points {
				points.push(egui::plot::Points::new(vec![[x, y]]).color(colour).highlight(true));
			}
			plot.show(ui, |plot_ui| {
				for point in points {
					plot_ui.points(point);
				}
			});
		});
	}
}
