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

					match self.chosen_task {
						Task::Task1 | Task::Task2 | Task::Task2Rotated | Task::Task3 | Task::Task4 | Task::Task5A | Task::Task5B | Task::Task7 => {
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
						}
						Task::Task6 => {}
					}

					match self.chosen_task {
						Task::Task1 | Task::Task2 | Task::Task2Rotated | Task::Task5A => {}
						Task::Task3 => {
							ui.add(egui::DragValue::new(&mut self.data.task_3_data.speed).speed(0.1));
							ui.label("Animation speed (years/second): ");
						}
						Task::Task4 => {
							ui.add(egui::DragValue::new(&mut self.data.task_4_data.speed).speed(0.1));
							ui.label("Animation speed (years/second): ");
						}
						Task::Task5B => {
							ui.add(egui::DragValue::new(&mut self.data.task_5b_data.speed).speed(0.1));
							ui.label("Animation speed (years/second): ");
						}
						Task::Task6 => {
							let mut any_changed = false;
							ui.menu_button("Settings", |ui| {
								ui.horizontal(|ui| {
									any_changed |= ui.add(egui::Slider::new(&mut self.data.task_6_data.dt, 0.0..=f64::INFINITY).logarithmic(true)).changed();
									ui.label("time step (years)");
								});
								ui.horizontal(|ui| {
									any_changed |= ui.add(egui::Slider::new(&mut self.data.task_6_data.number_of_periods, 0.0..=f64::INFINITY).logarithmic(true)).changed();
									ui.label("number of orbits of outer planet");
								});
								ui.horizontal(|ui| {
									any_changed |= ui.add(egui::Slider::new(&mut self.data.task_6_data.line_width, 0.0..=f32::INFINITY).logarithmic(true)).changed();
									ui.label("line width (pixels)");
								});
								ui.label("Objects (choose 2)");
								for (i, object) in self.planetary_systems[self.chosen_system].objects.iter().enumerate() {
									let mut checked = self.data.task_6_data.chosen_objects.contains(&i);
									ui.add_enabled_ui(checked || self.data.task_6_data.chosen_objects.len() < 2, |ui| {
										if ui.checkbox(&mut checked, &object.name).changed() {
											any_changed = true;
											if checked {
												if self.data.task_6_data.chosen_objects.is_empty() || self.data.task_6_data.chosen_objects[0] < i {
													self.data.task_6_data.chosen_objects.push(i);
												} else {
													self.data.task_6_data.chosen_objects.insert(0, i);
												}
											} else if self.data.task_6_data.chosen_objects[0] == i {
												self.data.task_6_data.chosen_objects.remove(0);
											} else {
												self.data.task_6_data.chosen_objects.remove(1);
											}
										}
									});
								}
							});
							if any_changed {
								self.data.init_task(&Task::Task6, self.chosen_system, &self.planetary_systems, &self.active_groups);
							}
						}
						Task::Task7 => {
							ui.menu_button("Settings", |ui| {
								ui.horizontal(|ui| {
									ui.add(egui::DragValue::new(&mut self.data.task_7_data.speed).speed(0.1));
									ui.label("Animation speed (years/second)");
								});
								ui.horizontal(|ui| {
									ui.add(egui::DragValue::new(&mut self.data.task_7_data.points_per_orbit).speed(0.1));
									ui.label("Points to keep per orbit of an object");
								});
							});
							let stationary_object_index = self.data.task_7_data.stationary_object_index;
							egui::ComboBox::from_id_source("Object to keep stationary")
								.selected_text(&self.planetary_systems[self.chosen_system].objects[self.data.task_7_data.stationary_object_index].name)
								.show_ui(ui, |ui: &mut egui::Ui| {
									ui.style_mut().wrap = Some(false);
									for (i, object) in self.planetary_systems[self.chosen_system].objects.iter().enumerate() {
										ui.selectable_value(&mut self.data.task_7_data.stationary_object_index, i, &object.name);
									}
								});
							if stationary_object_index != self.data.task_7_data.stationary_object_index {
								self.data.init_task(&Task::Task7, self.chosen_system, &self.planetary_systems, &self.active_groups);
							};
							ui.label("Object to keep stationary: ");
						}
					}
				});
			});
		})
	}
}
