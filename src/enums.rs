use std::fmt::Display;

pub const TASKS_NUM: usize = 4;
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Task {
	Task1,
	Task2,
	Task2Rotated,
	Task4,
}

impl Task {
	pub fn task_index(&self) -> usize {
		match *self {
			Self::Task1 => 0,
			Self::Task2 => 1,
			Self::Task2Rotated => 2,
			Self::Task4 => 3,
		}
	}

	pub fn from_index(index: usize) -> Self {
		match index {
			0 => Self::Task1,
			1 => Self::Task2,
			2 => Self::Task2Rotated,
			3 => Self::Task4,
			_ => todo!(),
		}
	}

	pub fn render_after_top_panel(&self) -> bool {
		match *self {
			Task::Task1 | Task::Task2 | Task::Task2Rotated => true,
			Task::Task4 => false,
		}
	}
}

impl Display for Task {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			Task::Task1 => write!(f, "Task 1"),
			Task::Task2 => write!(f, "Task 2"),
			Task::Task2Rotated => write!(f, "Task 2 with rotation"),
			Task::Task4 => write!(f, "Task 4"),
		}
	}
}
