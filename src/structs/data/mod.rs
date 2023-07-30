pub mod task_1;
pub mod task_2;
pub mod task_2_rotated;
pub mod task_3;
pub mod task_4;
pub mod task_5a;
pub mod task_5b;
pub mod task_6;
pub mod task_7;

use task_1::*;
use task_2::*;
use task_2_rotated::*;
use task_3::*;
use task_4::*;
use task_5a::*;
use task_5b::*;
use task_6::*;
use task_7::*;

use std::collections::HashMap;

use crate::{enums, structs};

pub struct Data {
	pub top_panel_bottom: f32,

	pub task_1_data: Task1Data,
	pub task_2_data: Task2Data,
	pub task_2_rotated_data: Task2RotatedData,
	pub task_3_data: Task3Data,
	pub task_4_data: Task4Data,
	pub task_5a_data: Task5AData,
	pub task_5b_data: Task5BData,
	pub task_6_data: Task6Data,
	pub task_7_data: Task7Data,
}

impl Data {
	pub fn new() -> Self {
		Self {
			top_panel_bottom: 0.0,

			task_1_data: Task1Data::new(),
			task_2_data: Task2Data::new(),
			task_2_rotated_data: Task2RotatedData::new(),
			task_3_data: Task3Data::new(),
			task_4_data: Task4Data::new(),
			task_5a_data: Task5AData::new(),
			task_5b_data: Task5BData::new(),
			task_6_data: Task6Data::new(),
			task_7_data: Task7Data::new(),
		}
	}

	pub fn init_task(&mut self, chosen_task: &enums::Task, chosen_system: usize, planetary_systems: &[structs::PlanetarySystem], active_groups: &[Vec<HashMap<String, bool>>]) {
		match *chosen_task {
			enums::Task::Task1 => self.init_task_1(&planetary_systems[chosen_system], &active_groups[chosen_task.task_index()][chosen_system]),
			enums::Task::Task2 => self.init_task_2(&planetary_systems[chosen_system], &active_groups[chosen_task.task_index()][chosen_system]),
			enums::Task::Task2Rotated => self.init_task_2_rotated(&planetary_systems[chosen_system], &active_groups[chosen_task.task_index()][chosen_system]),
			enums::Task::Task3 => self.init_task_3(&planetary_systems[chosen_system], &active_groups[chosen_task.task_index()][chosen_system]),
			enums::Task::Task4 => self.init_task_4(&planetary_systems[chosen_system], &active_groups[chosen_task.task_index()][chosen_system]),
			enums::Task::Task5A => self.init_task_5a(&planetary_systems[chosen_system], &active_groups[chosen_task.task_index()][chosen_system]),
			enums::Task::Task5B => self.init_task_5b(&planetary_systems[chosen_system], &active_groups[chosen_task.task_index()][chosen_system]),
			enums::Task::Task6 => self.init_task_6(&planetary_systems[chosen_system], &active_groups[chosen_task.task_index()][chosen_system]),
			enums::Task::Task7 => self.init_task_7(&planetary_systems[chosen_system], &active_groups[chosen_task.task_index()][chosen_system]),
		}
	}

	pub fn init_task_by_id(&mut self, chosen_task: usize, chosen_system: usize, planetary_systems: &Vec<structs::PlanetarySystem>, active_groups: &Vec<Vec<HashMap<String, bool>>>) {
		self.init_task(&enums::Task::from_index(chosen_task), chosen_system, planetary_systems, active_groups);
	}

	fn init_task_1(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		self.task_1_data.init(planetary_system, active_groups);
	}

	fn init_task_2(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		self.task_2_data.init(planetary_system, active_groups);
	}

	fn init_task_2_rotated(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		self.task_2_rotated_data.init(planetary_system, active_groups);
	}

	fn init_task_3(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		self.task_3_data.init(planetary_system, active_groups);
	}

	fn init_task_4(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		self.task_4_data.init(planetary_system, active_groups);
	}

	fn init_task_5a(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		self.task_5a_data.init(planetary_system, active_groups);
	}

	fn init_task_5b(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		self.task_5b_data.init(planetary_system, active_groups);
	}

	fn init_task_6(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		self.task_6_data.init(planetary_system, active_groups);
	}

	fn init_task_7(&mut self, planetary_system: &structs::PlanetarySystem, active_groups: &HashMap<String, bool>) {
		self.task_7_data.init(planetary_system, active_groups);
	}
}
