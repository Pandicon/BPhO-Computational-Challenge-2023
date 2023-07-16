use std::fmt::Display;

pub const TASKS_NUM: usize = 1;
#[derive(Eq, PartialEq)]
pub enum Task {
	Task1,
}

impl Task {
	pub fn task_index(&self) -> usize {
		match *self {
			Self::Task1 => 0,
		}
	}

	pub fn from_index(index: usize) -> Self {
		match index {
			0 => Self::Task1,
			_ => todo!(),
		}
	}
}

impl Display for Task {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			Task::Task1 => write!(f, "Task 1"),
		}
	}
}
