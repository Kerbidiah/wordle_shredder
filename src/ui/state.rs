use iced::Element;
use iced::{
	Align,
	button,
	Button,
	Sandbox,
	scrollable,
	Scrollable,
	Column,
	Container,
	Row,
	Text,
	text_input,
	TextInput,
	Settings,
	Length,
};
use iced::widget::{
	Space
};

use crate::list::*;
use super::LetterState;
use super::LMessage;
use super::design;

#[derive(Debug)]
pub struct State {
	// list stuff
	list: List,
	scroll: scrollable::State,

	// text input box thing
	input: text_input::State,
	input_value: String,

	// buttons
	submit: button::State, // for both button and text feild
	reset: button::State,
	eliminate: button::State,

	// letters
	ls0: LetterState,
	ls1: LetterState,
	ls2: LetterState,
	ls3: LetterState,
	ls4: LetterState,
}

#[derive(Debug, Clone)]
pub enum Message {
	Submit,
	Reset,
	Eliminate,
	InputChanged(String),
	LS0(LMessage),
	LS1(LMessage),
	LS2(LMessage),
	LS3(LMessage),
	LS4(LMessage),
}

impl State {
	pub fn show() {
		Self::run(Settings::default()).unwrap();
	}
}

impl Sandbox for State {

	type Message = Message;

	fn new() -> Self {
		State::default()
	}
	
	fn title(&self) -> String {
		String::from("Wordle Shredder")
	}

	fn update(&mut self, event: <Self as Sandbox>::Message) {
		match event {
			Message::Submit => {
				if self.input_value.len() == 5 {
					let lower = self.input_value.to_lowercase();
					let mut chars = lower.chars();
					self.ls0.set_letter(chars.next().unwrap());
					self.ls1.set_letter(chars.next().unwrap());
					self.ls2.set_letter(chars.next().unwrap());
					self.ls3.set_letter(chars.next().unwrap());
					self.ls4.set_letter(chars.next().unwrap());
				}
			},
			Message::Eliminate => { // TODO! only make it show up once all letters are chosen???
				self.list.execute(self.ls0.letter);
				self.list.execute(self.ls1.letter);
				self.list.execute(self.ls2.letter);
				self.list.execute(self.ls3.letter);
				self.list.execute(self.ls4.letter);

				self.input_value.clear();
				self.ls0.letter = Letter::Empty;
				self.ls1.letter = Letter::Empty;
				self.ls2.letter = Letter::Empty;
				self.ls3.letter = Letter::Empty;
				self.ls4.letter = Letter::Empty;
			},
			Message::Reset => {
				self.list = List::new();
			},
			Message::InputChanged(value) => {
				self.input_value = value;
			},
			Message::LS0(msg) => {
				self.ls0.set_color_and_index(msg, 0);
			},
			Message::LS1(msg) => {
				self.ls1.set_color_and_index(msg, 1);
			},
			Message::LS2(msg) => {
				self.ls2.set_color_and_index(msg, 2);
			},
			Message::LS3(msg) => {
				self.ls3.set_color_and_index(msg, 3);
			},
			Message::LS4(msg) => {
				self.ls4.set_color_and_index(msg, 4);
			},
		}
	}

	fn view(&mut self) -> iced::Element<'_, Self::Message> {
		// build the word list thingy
		let word_list: Element<_> = self.list.data.iter()
			.fold(Column::new().spacing(5), |column, word| {
				column.push(Text::new(word)) // TODO! multiple words per line
			})
		.into();

		let input_bar = {
			TextInput::new(
				&mut self.input,
				"word",
				&self.input_value,
				Message::InputChanged)
			.padding(15)
			.size(30)
			.on_submit(Message::Submit)
		};

		let letter_list: Element<_> = 
		Row::new()
			.push( // left side buffer
				Space::new(Length::Fill, Length::Shrink)
			)
			.push( // letter index 0
				Column::new()
					.push(Text::new(self.ls0.get_char()))
					.push(
						Button::new(&mut self.ls0.gray_button, Text::new("gray"))
						.on_press(Message::LS0(LMessage::GrayPressed))
						.style(design::ButtonGray)
					)
					.push(
						Button::new(&mut self.ls0.yellow_button, Text::new("yellow"))
						.on_press(Message::LS0(LMessage::YellowPressed))
						.style(design::ButtonYellow)
					)
					.push(
						Button::new(&mut self.ls0.green_button, Text::new("green"))
						.on_press(Message::LS0(LMessage::GreenPressed))
						.style(design::ButtonGreen)
					)
			)
			.push( // letter index 1
				Column::new()
					.push(Text::new(self.ls1.get_char()))
					.push(
						Button::new(&mut self.ls1.gray_button, Text::new("gray"))
						.on_press(Message::LS1(LMessage::GrayPressed))
						.style(design::ButtonGray)
					)
					.push(
						Button::new(&mut self.ls1.yellow_button, Text::new("yellow"))
						.on_press(Message::LS1(LMessage::YellowPressed))
						.style(design::ButtonYellow)
					)
					.push(
						Button::new(&mut self.ls1.green_button, Text::new("green"))
						.on_press(Message::LS1(LMessage::GreenPressed))
						.style(design::ButtonGreen)
					)
			)
			.push( // letter index 2
				Column::new()
					.push(Text::new(self.ls2.get_char()))
					.push(
						Button::new(&mut self.ls2.gray_button, Text::new("gray"))
						.on_press(Message::LS2(LMessage::GrayPressed))
						.style(design::ButtonGray)
					)
					.push(
						Button::new(&mut self.ls2.yellow_button, Text::new("yellow"))
						.on_press(Message::LS2(LMessage::YellowPressed))
						.style(design::ButtonYellow)
					)
					.push(
						Button::new(&mut self.ls2.green_button, Text::new("green"))
						.on_press(Message::LS2(LMessage::GreenPressed))
						.style(design::ButtonGreen)
					)
			)
			.push( // letter index 3
				Column::new()
					.push(Text::new(self.ls3.get_char()))
					.push(
						Button::new(&mut self.ls3.gray_button, Text::new("gray"))
						.on_press(Message::LS3(LMessage::GrayPressed))
						.style(design::ButtonGray)
					)
					.push(
						Button::new(&mut self.ls3.yellow_button, Text::new("yellow"))
						.on_press(Message::LS3(LMessage::YellowPressed))
						.style(design::ButtonYellow)
					)
					.push(
						Button::new(&mut self.ls3.green_button, Text::new("green"))
						.on_press(Message::LS3(LMessage::GreenPressed))
						.style(design::ButtonGreen)
					)
			)
			.push( // letter index 4
				Column::new()
					.push(Text::new(self.ls4.get_char()))
					.push(
						Button::new(&mut self.ls4.gray_button, Text::new("gray"))
						.on_press(Message::LS4(LMessage::GrayPressed))
						.style(design::ButtonGray)
					)
					.push(
						Button::new(&mut self.ls4.yellow_button, Text::new("yellow"))
						.on_press(Message::LS4(LMessage::YellowPressed))
						.style(design::ButtonYellow)
					)
					.push(
						Button::new(&mut self.ls4.green_button, Text::new("green"))
						.on_press(Message::LS4(LMessage::GreenPressed))
						.style(design::ButtonGreen)
					)
			)
			.push( // right side buffer
				Space::new(Length::Fill, Length::Shrink)
			)
		.into();
		

		Row::new() // split screen left & right
			.padding(20)
			.align_items(Align::Start)
			// left side of screen: text input and stuff
			.push(
				Column::new()
					.width(Length::FillPortion(4))
					.padding(20)	
					.push(
						Row::new() // input and submit row
							.spacing(20)
							.push(
								input_bar
								.width(Length::Fill)
							)
							.push(
								Button::new(&mut self.submit, Text::new("submit"))
								.on_press(Message::Submit)
								.width(Length::Shrink)
							)
					)
					.push(letter_list)
					.push(Space::new(Length::Shrink ,Length::Fill))
					.push(
						Button::new(&mut self.eliminate, Text::new("eliminate"))
						.on_press(Message::Eliminate)
					)
			)

			// right side of screen: the list
			.push(
				Column::new()
				.width(Length::FillPortion(1))
				.spacing(20)
				.padding(0)
				.push(Text::new(self.list.stats()))
				.push(
					Button::new(&mut self.reset, Text::new("reset"))
					.on_press(Message::Reset) // , // idk why comma
				)
				.push( // the list
					Scrollable::new(&mut self.scroll)
					.push(word_list)
				)
			)
			.into()
	}
}

impl Default for State {
	fn default() -> Self {
		Self {
			list: List::new(),
			scroll: scrollable::State::new(),
			input: text_input::State::new(),
    		input_value: String::from(""),
			submit: button::State::new(),
			reset: button::State::new(),
			eliminate: button::State::new(),
			ls0: LetterState::new(0),
			ls1: LetterState::new(1),
			ls2: LetterState::new(2),
			ls3: LetterState::new(3),
			ls4: LetterState::new(4),
		}
	}
}