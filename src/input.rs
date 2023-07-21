use eframe::egui;

use crate::enums::Task;

pub struct Input {
	pub dragged: egui::Vec2,
	pub dragged_rotate: egui::Vec2,
	pub zoom: f32,
}

impl Default for Input {
	fn default() -> Self {
		Self {
			dragged: egui::Vec2::new(0.0, 0.0),
			dragged_rotate: egui::Vec2::new(0.0, 0.0),
			zoom: 0.0,
		}
	}
}

impl crate::application::Application {
	pub fn handle_input(&mut self, input: Input) {
		match self.chosen_task {
			Task::Task1 | Task::Task2 | Task::Task2Rotated => {}
			Task::Task4 => {
				self.data.task_4_data.offset_x += input.dragged.x;
				self.data.task_4_data.offset_y += input.dragged.y;
				self.data.task_4_data.rotate_x += input.dragged_rotate.y;
				self.data.task_4_data.rotate_y += input.dragged_rotate.x;
				self.data.task_4_data.zoom_coefficient += input.zoom;

				if self.data.task_4_data.rotate_x > 90.0 {
					self.data.task_4_data.rotate_x = 90.0;
				} else if self.data.task_4_data.rotate_x < -90.0 {
					self.data.task_4_data.rotate_x = -90.0;
				}
			}
		}
	}

	pub fn get_input(&mut self, ctx: &egui::Context) -> Input {
		let input_events = ctx.input(|i| i.events.clone());
		let mut input = Input::default();
		if ctx.input(|i| i.pointer.secondary_down()) {
			input.dragged_rotate = ctx.input(|i: &egui::InputState| i.pointer.delta());
		} else if ctx.input(|i| i.pointer.primary_down()) {
			input.dragged = ctx.input(|i: &egui::InputState| i.pointer.delta());
		}
		for event in &input_events {
			match *event {
				egui::Event::Zoom(zoom) => {
					if zoom < 1.0 {
						input.zoom = -0.25;
					} else if zoom == 1.0 {
						input.zoom = 0.0;
					} else {
						input.zoom = 0.25;
					}
				}
				egui::Event::Scroll(egui::Vec2 { y, .. }) => {
					if y < 0.0 {
						input.zoom = -0.25;
					} else if y == 0.0 {
						input.zoom = 0.0;
					} else {
						input.zoom = 0.25;
					}
				}
				_ => {}
			}
		}
		input
	}
}
