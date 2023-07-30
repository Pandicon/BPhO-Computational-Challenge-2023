use std::fmt::Display;

pub const TASKS_NUM: usize = 10;
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Task {
	Task1,
	Task2,
	Task2Rotated,
	Task3,
	Task4,
	Task5A,
	Task5B,
	Task5C,
	Task6,
	Task7,
}

impl Task {
	pub fn task_index(&self) -> usize {
		match *self {
			Self::Task1 => 0,
			Self::Task2 => 1,
			Self::Task2Rotated => 2,
			Self::Task3 => 3,
			Self::Task4 => 4,
			Self::Task5A => 5,
			Self::Task5B => 6,
			Self::Task5C => 7,
			Self::Task6 => 8,
			Self::Task7 => 9,
		}
	}

	pub fn from_index(index: usize) -> Self {
		match index {
			0 => Self::Task1,
			1 => Self::Task2,
			2 => Self::Task2Rotated,
			3 => Self::Task3,
			4 => Self::Task4,
			5 => Self::Task5A,
			6 => Self::Task5B,
			7 => Self::Task5C,
			8 => Self::Task6,
			9 => Self::Task7,
			_ => todo!(),
		}
	}

	pub fn render_after_top_panel(&self) -> bool {
		match *self {
			Self::Task1 | Self::Task2 | Self::Task2Rotated | Self::Task3 | Self::Task5A | Self::Task5B => true,
			Self::Task4 | Self::Task5C | Self::Task6 | Self::Task7 => false,
		}
	}

	pub fn should_request_repaint(&self) -> bool {
		match *self {
			Self::Task3 | Self::Task4 | Self::Task5B | Task::Task5C | Self::Task7 => true,
			Self::Task1 | Self::Task2 | Self::Task2Rotated | Self::Task5A | Self::Task6 => false,
		}
	}
}

impl Display for Task {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			Self::Task1 => write!(f, "Task 1"),
			Self::Task2 => write!(f, "Task 2"),
			Self::Task2Rotated => write!(f, "Task 2 with rotation"),
			Self::Task3 => write!(f, "Task 3"),
			Self::Task4 => write!(f, "Task 4"),
			Self::Task5A => write!(f, "Task 5a"),
			Self::Task5B => write!(f, "Task 5b"),
			Self::Task5C => write!(f, "Task 5c"),
			Self::Task6 => write!(f, "Task 6"),
			Self::Task7 => write!(f, "Task 7"),
		}
	}
}
