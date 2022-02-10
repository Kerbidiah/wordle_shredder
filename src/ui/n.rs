// settings for UI stuff

pub mod button {
	// use super::*;

	pub const SIZE: u16 = 30;
	pub const SPACING: u16 = 5;
	pub const PADDING: u16 = SPACING * 2;
}

pub mod text_input {
	use super::*;

	pub mod spacing {
		use super::*;

		pub const GAP: u16 = button::SPACING * 3;
		pub const PORTION: u16 = 7;
	}
}