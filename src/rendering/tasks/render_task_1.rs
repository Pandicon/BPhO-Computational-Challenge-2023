use eframe::egui;

use crate::application::Application;

impl Application {
	pub fn render_task_1(&self, ctx: &egui::Context) {
		egui::CentralPanel::default().show(ctx, |ui| {
			let plot = egui::plot::Plot::new("T-A relationship").data_aspect(1.0);
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
