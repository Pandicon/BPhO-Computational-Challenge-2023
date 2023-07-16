use eframe::egui;

use crate::{application, enums::Task};

impl application::Application {
	pub fn render_top_panel(&mut self, ctx: &egui::Context) -> egui::InnerResponse<()> {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
					ui.label("BPhO Computation Challenge 2023");
				});
				ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
					egui::ComboBox::from_id_source("Task to show: ")
						.selected_text(format!("{}", self.chosen_task))
						.show_ui(ui, |ui: &mut egui::Ui| {
							ui.style_mut().wrap = Some(false);
							ui.selectable_value(&mut self.chosen_task, Task::Task1, format!("{}", Task::Task1));
						});
					ui.label("Task to show: ");
				});
			});
		})
	}
}
