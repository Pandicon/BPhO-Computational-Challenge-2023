use std::fmt::Display;

#[derive(Eq, PartialEq)]
pub enum Task {
	Task1,
}

impl Display for Task {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			Task::Task1 => write!(f, "Task 1"),
		}
	}
}
