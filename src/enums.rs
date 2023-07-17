use std::fmt::Display;

pub const TASKS_NUM: usize = 3;
#[derive(Eq, PartialEq)]
pub enum Task {
	Task1,
	Task2,
	Task2Rotated,
}

impl Task {
	pub fn task_index(&self) -> usize {
		match *self {
			Self::Task1 => 0,
			Self::Task2 => 1,
			Self::Task2Rotated => 2,
		}
	}

	pub fn from_index(index: usize) -> Self {
		match index {
			0 => Self::Task1,
			1 => Self::Task2,
			2 => Self::Task2Rotated,
			_ => todo!(),
		}
	}
}

impl Display for Task {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			Task::Task1 => write!(f, "Task 1"),
			Task::Task2 => write!(f, "Task 2"),
			Task::Task2Rotated => write!(f, "Task 2 with rotation"),
		}
	}
}
