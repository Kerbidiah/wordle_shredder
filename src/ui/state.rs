use iced::{
	Button, button,
	Column, Row,
	Container,
	Element,
	Length,
	Sandbox,
	Scrollable, scrollable,
	Settings,
	Text,
	TextInput, text_input,
};

use iced::widget::Space;
use iced::alignment::{self, Alignment};

use crate::list::*;
use super::LetterState;
use super::LMessage;
use super::design;
use super::n;

// all the state for the app
// the app can be thought of as representing and changing all the stuff in here
#[derive(Debug)]
pub struct State {
	// list stuff
	list: List, // the list of words the user can see
	scroll: scrollable::State, // this is needed for the scrolling to work

	// text input box thing
	input: text_input::State, // the state of input box itself
	input_value: String, // the string inputted

	// button states
	submit: button::State, // used for both button and text field
	reset: button::State,
	eliminate: button::State,

	// letters
	ls0: LetterState, // see letter_state.rs
	ls1: LetterState,
	ls2: LetterState,
	ls3: LetterState,
	ls4: LetterState,
}

// all the things the app can do
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
	// this is the function called in main.rs that actually makes the app run
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
				self.ls0.letter = Letter::Gray(' ');
				self.ls1.letter = Letter::Gray(' ');
				self.ls2.letter = Letter::Gray(' ');
				self.ls3.letter = Letter::Gray(' ');
				self.ls4.letter = Letter::Gray(' ');
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
			.spacing(n::button::PADDING)
			.push( // letter index 0
				Column::new()
					.spacing(n::button::SPACING)
					.width(Length::Fill)
					.align_items(Alignment::Center)
					.push(
						Container::new(
							Text::new(self.ls0.get_char_upper())
							.size(n::button::SIZE)
						)
						.style(design::container_color::dynamic(&self.ls0))
						.height(Length::Fill).width(Length::Fill)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center)
					)
					.push(
						Button::new(
							&mut self.ls0.gray_button,
							Text::new("gray")
							.horizontal_alignment(alignment::Horizontal::Center)
							.vertical_alignment(alignment::Vertical::Center)
                            .size(n::button::SIZE2)
						)
						.on_press(Message::LS0(LMessage::GrayPressed))
						.style(design::ButtonGray)
						.height(Length::Fill).width(Length::Fill)
					)
					.push(
						Button::new(
							&mut self.ls0.yellow_button,
							Text::new("yellow")
							.horizontal_alignment(alignment::Horizontal::Center)
							.vertical_alignment(alignment::Vertical::Center)
                            .size(n::button::SIZE2)
						)
						.on_press(Message::LS0(LMessage::YellowPressed))
						.style(design::ButtonYellow)
						.height(Length::Fill).width(Length::Fill)
					)
					.push(
						Button::new(
							&mut self.ls0.green_button,
							Text::new("green")
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center)
                            .size(n::button::SIZE2)
						)
						.on_press(Message::LS0(LMessage::GreenPressed))
						.style(design::ButtonGreen)
						.height(Length::Fill).width(Length::Fill)
					)
			)
			.push( // letter index 1
				Column::new()
					.spacing(n::button::SPACING)
					.width(Length::Fill)
					.align_items(Alignment::Center)
					.push(
						Container::new(
							Text::new(self.ls1.get_char_upper())
							.size(n::button::SIZE)
						)
						.style(design::container_color::dynamic(&self.ls1))
						.height(Length::Fill).width(Length::Fill)
						.align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center)
					)
					//.push(Space::new(Length::Fill, Length::Fill)) // IDK how big to make the space
					.push(
						Button::new(
							&mut self.ls1.gray_button,
							Text::new("gray")
							.horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center)
                            .size(n::button::SIZE2)
						)
						.on_press(Message::LS1(LMessage::GrayPressed))
						.style(design::ButtonGray)
						.height(Length::Fill).width(Length::Fill)
					)
					.push(
						Button::new(
							&mut self.ls1.yellow_button,
							Text::new("yellow")
							.horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center)
                            .size(n::button::SIZE2)
						)
						.on_press(Message::LS1(LMessage::YellowPressed))
						.style(design::ButtonYellow)
						.height(Length::Fill).width(Length::Fill)
					)
					.push(
						Button::new(
							&mut self.ls1.green_button,
							Text::new("green")
							.horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center)
                            .size(n::button::SIZE2)
						)
						.on_press(Message::LS1(LMessage::GreenPressed))
						.style(design::ButtonGreen)
						.height(Length::Fill).width(Length::Fill)
					)
			)
			.push( // letter index 2
				Column::new()
					.spacing(n::button::SPACING)
					.width(Length::Fill)
					.align_items(Alignment::Center)
					.push(
						Container::new(
							Text::new(self.ls2.get_char_upper())
							.size(n::button::SIZE)
						)
						.style(design::container_color::dynamic(&self.ls2))
						.height(Length::Fill).width(Length::Fill)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center)
					)
					.push(
						Button::new(
							&mut self.ls2.gray_button,
							Text::new("gray")
							.horizontal_alignment(alignment::Horizontal::Center)
							.vertical_alignment(alignment::Vertical::Center)
                            .size(n::button::SIZE2)
						)
						.on_press(Message::LS2(LMessage::GrayPressed))
						.style(design::ButtonGray)
						.height(Length::Fill).width(Length::Fill)
					)
					.push(
						Button::new(
							&mut self.ls2.yellow_button,
							Text::new("yellow")
							.horizontal_alignment(alignment::Horizontal::Center)
							.vertical_alignment(alignment::Vertical::Center)
                            .size(n::button::SIZE2)
						)
						.on_press(Message::LS2(LMessage::YellowPressed))
						.style(design::ButtonYellow)
						.height(Length::Fill).width(Length::Fill)
					)
					.push(
						Button::new(
							&mut self.ls2.green_button,
							Text::new("green")
							.horizontal_alignment(alignment::Horizontal::Center)
							.vertical_alignment(alignment::Vertical::Center)
                            .size(n::button::SIZE2)
						)
						.on_press(Message::LS2(LMessage::GreenPressed))
						.style(design::ButtonGreen)
						.height(Length::Fill).width(Length::Fill)
					)
			)
			.push( // letter index 3
				Column::new()
					.spacing(n::button::SPACING)
					.width(Length::Fill)
					.align_items(Alignment::Center)
					.push(
						Container::new(
							Text::new(self.ls3.get_char_upper())
							.size(n::button::SIZE)
						)
						.style(design::container_color::dynamic(&self.ls3))
						.height(Length::Fill).width(Length::Fill)
						.align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center)
					)
					.push(
						Button::new(
							&mut self.ls3.gray_button,
							Text::new("gray")
							.horizontal_alignment(alignment::Horizontal::Center)
							.vertical_alignment(alignment::Vertical::Center)
                            .size(n::button::SIZE2)
						)
						.on_press(Message::LS3(LMessage::GrayPressed))
						.style(design::ButtonGray)
						.height(Length::Fill).width(Length::Fill)
					)
					.push(
						Button::new(
							&mut self.ls3.yellow_button,
							Text::new("yellow")
							.horizontal_alignment(alignment::Horizontal::Center)
							.vertical_alignment(alignment::Vertical::Center)
                            .size(n::button::SIZE2)
						)
						.on_press(Message::LS3(LMessage::YellowPressed))
						.style(design::ButtonYellow)
						.height(Length::Fill).width(Length::Fill)
					)
					.push(
						Button::new(
							&mut self.ls3.green_button,
							Text::new("green")
							.horizontal_alignment(alignment::Horizontal::Center)
							.vertical_alignment(alignment::Vertical::Center)
                            .size(n::button::SIZE2)
						)
						.on_press(Message::LS3(LMessage::GreenPressed))
						.style(design::ButtonGreen)
						.height(Length::Fill).width(Length::Fill)
					)
			)
			.push( // letter index 4
				Column::new()
					.spacing(n::button::SPACING)
					.width(Length::Fill)
					.align_items(Alignment::Center)
					.push(
						Container::new(
							Text::new(self.ls4.get_char_upper())
							.size(n::button::SIZE)
						)
						.style(design::container_color::dynamic(&self.ls4))
						.height(Length::Fill).width(Length::Fill)
						.align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center)
					)
					.push(
						Button::new(
							&mut self.ls4.gray_button,
							Text::new("gray")
							.horizontal_alignment(alignment::Horizontal::Center)
							.vertical_alignment(alignment::Vertical::Center)
                            .size(n::button::SIZE2)
						)
						.on_press(Message::LS4(LMessage::GrayPressed))
						.style(design::ButtonGray)
						.height(Length::Fill).width(Length::Fill)
					)
					.push(
						Button::new(
							&mut self.ls4.yellow_button,
							Text::new("yellow")
							.horizontal_alignment(alignment::Horizontal::Center)
							.vertical_alignment(alignment::Vertical::Center)
                            .size(n::button::SIZE2)
						)
						.on_press(Message::LS4(LMessage::YellowPressed))
						.style(design::ButtonYellow)
						.height(Length::Fill).width(Length::Fill)
					)
					.push(
						Button::new(
							&mut self.ls4.green_button,
							Text::new("green")
							.horizontal_alignment(alignment::Horizontal::Center)
							.vertical_alignment(alignment::Vertical::Center)
                            .size(n::button::SIZE2)
						)
						.on_press(Message::LS4(LMessage::GreenPressed))
						.style(design::ButtonGreen)
						.height(Length::Fill).width(Length::Fill)
					)
			)
		.into();
		

		Row::new() // split screen left & right
			.padding(20)
			.align_items(Alignment::Start)
			// left side of screen: text input and stuff
			.push(
				Column::new()
					.width(Length::FillPortion(4))
					.padding(20)	
					.push(
						Row::new() // input and submit row
							.spacing(20)
							.push(
								Column::new()
									.width(Length::FillPortion(n::text_input::spacing::PORTION))
									.spacing(n::text_input::spacing::GAP)
									.push(input_bar)
									.push(letter_list)
							)
							.push(
								Button::new(&mut self.submit, Text::new("submit").width(Length::FillPortion(1)))
								.on_press(Message::Submit)
							)
					)
					.push(Space::new(Length::Shrink, Length::Fill))
					.push( // TODO: move eliminate button FIXME: its tiny
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