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
					if ui.button("Show loaded planetary systems").clicked() {
						self.show_loaded_systems = true;
					}
					egui::ComboBox::from_id_source("Task to show: ")
						.selected_text(format!("{}", self.chosen_task))
						.show_ui(ui, |ui: &mut egui::Ui| {
							ui.style_mut().wrap = Some(false);

							for task_i in 0..crate::enums::TASKS_NUM {
								let task = Task::from_index(task_i);
								ui.selectable_value(&mut self.chosen_task, task, format!("{}", task));
							}
						});
					ui.label("Task to show: ");

					egui::ComboBox::from_id_source("Planetary system to use: ")
						.selected_text(&self.planetary_systems[self.chosen_system].name)
						.show_ui(ui, |ui: &mut egui::Ui| {
							ui.style_mut().wrap = Some(false);
							for (i, system) in self.planetary_systems.iter().enumerate() {
								ui.selectable_value(&mut self.chosen_system, i, &system.name);
							}
						});
					ui.label("Planetary system to use: ");
					ui.menu_button("Object groups to display", |ui| {
						let mut any_changed = false;
						let _ = ui.button("Choose which groups of objects should be displayed in this task");
						let mut key_value_pairs = Vec::new();
						for (key, value) in self.active_groups[self.chosen_task.task_index()][self.chosen_system].iter_mut() {
							key_value_pairs.push((key, value));
						}
						key_value_pairs.sort_by(|a, b| a.0.cmp(b.0));
						for (key, value) in key_value_pairs {
							any_changed |= ui.checkbox(value, key).changed();
						}
						if any_changed {
							self.data.init_task(&self.chosen_task, self.chosen_system, &self.planetary_systems, &self.active_groups);
						}
					});

					match self.chosen_task {
						Task::Task1 | Task::Task2 | Task::Task2Rotated | Task::Task5A => {}
						Task::Task4 => {
							ui.add(egui::DragValue::new(&mut self.data.task_4_data.speed).speed(0.1));
							ui.label("Animation speed (years/second): ");
						}
					}
				});
			});
		})
	}
}
