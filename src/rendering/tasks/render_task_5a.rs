use std::ops::RangeInclusive;

use eframe::egui;

use crate::application::Application;

impl Application {
	pub fn render_task_5a(&mut self, ctx: &egui::Context) {
		egui::CentralPanel::default().show(ctx, |ui| {
			let x_axis_fmt = |val: f64, _range: &RangeInclusive<f64>| format!("{:.3} year{}", val, if val == 1.0 { "" } else { "s" });
			let y_axis_fmt = |val: f64, _range: &RangeInclusive<f64>| format!("{:.3} rad{}", val, if val == 1.0 { "" } else { "s" });

			let label_fmt = |_s: &str, val: &egui::plot::PlotPoint| {
				format!(
					"x: {:.3} year{}\ny: {:.3} rad{}",
					val.x,
					if val.x == 1.0 { "" } else { "s" },
					val.y,
					if val.y == 1.0 { "" } else { "s" }
				)
			};

			let plot = egui::plot::Plot::new("Orbit angle vs time")
				.x_axis_formatter(x_axis_fmt)
				.y_axis_formatter(y_axis_fmt)
				.label_formatter(label_fmt)
				.legend(egui::plot::Legend::default());

			let mut lines = Vec::new();
			for (points, colour, index, name, add_marker) in &self.data.task_5a_data.points {
				let (&index, &colour, &dashed) = (index, colour, add_marker);
				let mut orbit_points = Vec::new();
				for &[t, theta] in points {
					orbit_points.push([t, theta]);
				}
				lines.push(
					egui::plot::Line::new(orbit_points)
						.color(colour)
						.highlight(true)
						.style(if dashed { egui::plot::LineStyle::dashed_dense() } else { egui::plot::LineStyle::Solid })
						.name(format!("[{}] {}", index, name.to_owned())),
				);
			}
			plot.show(ui, |plot_ui| {
				for line in lines {
					plot_ui.line(line);
				}

				plot_ui.plot_bounds()
			});
		});
	}
}
