
#[derive(Debug, Clone, Copy)]
pub enum Letter {
	Empty,
	Blank(char),
	Gray(char),
	Yellow(char, usize),
	Green(char, usize),
}

impl Letter {
	pub fn get_char(&self) -> Option<char> {
		match self {
			Self::Empty => {
				None
			},
			Self::Blank(c) => {
				Some(*c)
			},
			Self::Gray(c) => {
				Some(*c)
			},
			Self::Yellow(c, _) => {
				Some(*c)
			},
			Self::Green(c, _) => {
				Some(*c)
			},
		}
	}
}