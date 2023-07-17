use eframe::{egui, epaint::Color32};

use crate::application::Application;

impl Application {
	pub fn render_loaded_systems_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
		egui::Window::new("Loaded planetary systems").open(&mut self.show_loaded_systems).show(ctx, |ui| {
			let mut any_colour_changed = false;
			egui::ScrollArea::vertical().show(ui, |ui| {
				for system in &mut self.planetary_systems {
					egui::CollapsingHeader::new(egui::RichText::new(&system.name).text_style(egui::TextStyle::Heading).size(20.0))
						.default_open(true)
						.show(ui, |ui| {
							let table = egui_extras::TableBuilder::new(ui)
								.striped(true)
								.resizable(true)
								.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
								.column(egui_extras::Column::auto())
								.column(egui_extras::Column::auto())
								.column(egui_extras::Column::auto())
								.column(egui_extras::Column::auto())
								.column(egui_extras::Column::auto())
								.column(egui_extras::Column::auto())
								.column(egui_extras::Column::auto())
								.column(egui_extras::Column::auto())
								.column(egui_extras::Column::auto())
								.column(egui_extras::Column::auto())
								.column(egui_extras::Column::auto())
								.min_scrolled_height(0.0);
							table
								.header(20.0, |mut header| {
									header.col(|ui| {
										ui.strong("Object");
									});
									header.col(|ui| {
										ui.strong("Semi-major axis (km)");
									});
									header.col(|ui| {
										ui.strong("Semi-major axis (AU)");
									});
									header.col(|ui| {
										ui.strong("Eccentricity");
									});
									header.col(|ui| {
										ui.strong("Inclination (degrees)");
									});
									header.col(|ui| {
										ui.strong("Mean longitude (degrees)");
									});
									header.col(|ui| {
										ui.strong("Longitude of perihelion (degrees)");
									});
									header.col(|ui| {
										ui.strong("Longitude of the ascending node (degrees)");
									});
									header.col(|ui| {
										ui.strong("Period (years)");
									});
									header.col(|ui| {
										ui.strong("Group");
									});
									header.col(|ui| {
										ui.strong("Colour");
									});
								})
								.body(|mut body| {
									for object in &mut system.objects {
										body.row(18.0, |mut row| {
											row.col(|ui| {
												ui.label(&object.name);
											});
											row.col(|ui| {
												ui.label(format!("{:.3}", object.distance_km));
											});
											row.col(|ui| {
												ui.label(format!("{:.8}", object.distance_au));
											});
											row.col(|ui| {
												ui.label(format!("{:.7}", object.eccentricity));
											});
											row.col(|ui| {
												ui.label(format!("{:.7}", object.inclination));
											});
											row.col(|ui| {
												ui.label(format!("{:.7}", object.mean_longitude));
											});
											row.col(|ui| {
												ui.label(format!("{:.7}", object.longitude_of_perihelion));
											});
											row.col(|ui| {
												ui.label(format!("{:.7}", object.longitude_of_ascending_node));
											});
											row.col(|ui| {
												ui.label(format!("{:.4}", object.period_years));
											});
											row.col(|ui| {
												ui.label(&object.group);
											});
											row.col(|ui| {
												let mut rgba = [
													(object.colour.r() as f32) / 255.0,
													(object.colour.g() as f32) / 255.0,
													(object.colour.b() as f32) / 255.0,
													(object.colour.a() as f32) / 255.0,
												];
												if ui.color_edit_button_rgba_unmultiplied(&mut rgba).changed() {
													object.colour = Color32::from_rgba_unmultiplied((rgba[0] * 255.0) as u8, (rgba[1] * 255.0) as u8, (rgba[2] * 255.0) as u8, (rgba[3] * 255.0) as u8);
													any_colour_changed = true;
												}
											});
										});
									}
								});
						});
				}
			});
			if any_colour_changed {
				for task_i in 0..crate::enums::TASKS_NUM {
					self.data.init_task_by_id(task_i, self.chosen_system, &self.planetary_systems, &self.active_groups);
				}
			}
		})
	}
}
