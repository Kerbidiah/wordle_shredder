use crate::list::Letter;
use iced::button;

#[derive(Debug, Clone, Copy)]
pub struct LetterState {
	pub index: usize,
	pub letter: Letter,

	// the buttons for choosing each letter
	pub gray_button: button::State,
	pub yellow_button: button::State,
	pub green_button: button::State,
}

impl LetterState {
	pub fn new(i: usize) -> Self {
		LetterState {
			index: i,
			letter: Letter::Gray(' '),
			gray_button: button::State::new(),
			yellow_button: button::State::new(),
			green_button: button::State::new(),
		}
	}

	pub fn set_letter(&mut self, c: char) {
		self.letter = Letter::Gray(c);
	}

	pub fn set_color_and_index(&mut self, color: LMessage, i: usize) {
		match color {
			LMessage::GrayPressed => {
				self.letter = Letter::Gray(self.get_char());
			}
			LMessage::YellowPressed => {
				self.letter = Letter::Yellow(self.get_char(), i);
			}
			LMessage::GreenPressed => {
				self.letter = Letter::Green(self.get_char(), i);
			}
		}
	}

	pub fn get_char(&self) -> char {
		let x = self.letter.get_char();

		if let Some(c) = x {
			c
		} else {
			' '
		}
	}

	pub fn get_char_upper(&self) -> char {
		let x = self.letter.get_char();

		if let Some(c) = x {
			c.to_ascii_uppercase()
		} else {
			' '
		}
	}

	// pub fn get_string(&self) -> String {
	// 	String::from(self.get_char())
	// }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy)]
pub enum LMessage {
	GrayPressed,
	YellowPressed,
	GreenPressed,
}
