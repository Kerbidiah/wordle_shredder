#![allow(dead_code)]

use iced::{button, container, Color};

const BORDER_RADIUS: f32 = 4.0;
const BORDER_WIDTH: f32 = 1.0;
const TEXT_COLOR: Color = Color::WHITE;

const GREEN: Color = Color::from_rgb(
	0x74 as f32 / 255.0,
	0xA7 as f32 / 255.0,
	0x5F as f32 / 255.0,
);

const YELLOW: Color = Color::from_rgb(
	0xE1 as f32 / 255.0,
	0xB6 as f32 / 255.0,
	0x3E as f32 / 255.0,
);

const GRAY: Color = Color::from_rgb(
	0x62 as f32 / 255.0,
	0x66 as f32 / 255.0,
	0x83 as f32 / 255.0,
);

const BLANK: Color = Color::from_rgb(
	0xE7 as f32 / 255.0, 
	0xE8 as f32 / 255.0,
	0xED as f32 / 255.0
);

pub struct BlankContainer;

impl container::StyleSheet for BlankContainer {
	fn style(&self) -> container::Style {
		container::Style {
			text_color: Color::WHITE.into(),
			background: BLANK.into(),
			border_radius: BORDER_RADIUS,
			..container::Style::default()
		}
	}
}

pub struct GreenContainer;

impl container::StyleSheet for GreenContainer {
	fn style(&self) -> container::Style {
		container::Style {
			text_color: TEXT_COLOR.into(),
			background: GREEN.into(),
			border_radius: BORDER_RADIUS,
			..container::Style::default()
		}
	}
}

pub struct YellowContainer;

impl container::StyleSheet for YellowContainer {
	fn style(&self) -> container::Style {
		container::Style {
			text_color: TEXT_COLOR.into(),
			background: YELLOW.into(),
			border_radius: BORDER_RADIUS,
			..container::Style::default()
		}
	}
}

pub struct GrayContainer;

impl container::StyleSheet for GrayContainer {
	fn style(&self) -> container::Style {
		container::Style {
			text_color: TEXT_COLOR.into(),
			background: GRAY.into(),
			border_radius: BORDER_RADIUS,
			..container::Style::default()
		}
	}
}

pub struct ButtonGreen;

impl button::StyleSheet for ButtonGreen {
	fn active(&self) -> button::Style {
		button::Style {
			border_radius: BORDER_RADIUS,
			text_color: TEXT_COLOR.into(),
			background: GREEN.into(),
			..button::Style::default()
		}
	}

	fn hovered(&self) -> button::Style {
		button::Style {
			..self.active()
		}
	}

	fn pressed(&self) -> button::Style {
		button::Style {
			..self.hovered()
		}
	}
}

pub struct ButtonYellow;

impl button::StyleSheet for ButtonYellow {
	fn active(&self) -> button::Style {
		button::Style {
			border_radius: BORDER_RADIUS,
			text_color: TEXT_COLOR.into(),
			background: YELLOW.into(),
			..button::Style::default()
		}
	}

	fn hovered(&self) -> button::Style {
		button::Style {
			..self.active()
		}
	}

	fn pressed(&self) -> button::Style {
		button::Style {
			..self.hovered()
		}
	}
}

pub struct ButtonGray;

impl button::StyleSheet for ButtonGray {
	fn active(&self) -> button::Style {
		button::Style {
			border_radius: BORDER_RADIUS,
			text_color: TEXT_COLOR.into(),
			background: GRAY.into(),
			..button::Style::default()
		}
	}

	fn hovered(&self) -> button::Style {
		button::Style {
			..self.active()
		}
	}

	fn pressed(&self) -> button::Style {
		button::Style {
			..self.hovered()
		}
	}
}