use rayon::prelude::*;

use super::Letter;

#[derive(Debug, Clone)]
pub struct List {
	pub data: Vec<String>,
	og_len: usize
}

impl List {
	pub fn new() -> Self {
		let cnts = include_str!("../../big_list.nooooo").to_ascii_lowercase();

		let data: Vec<String> = cnts.split(&['\n', ' '][..]).par_bridge().map(|x| x.to_string()).collect();
		let og_len = data.len();
		
		// should these be enabled for release builds too???
		debug_assert!(&data.par_iter().all(|word| (word.len() == 5) && (word.is_ascii())));

		Self {
			data,
			og_len,
		}
	}

	pub fn len(&self) -> usize {
		self.data.len()
	}

	pub fn stats(&self) -> String {
		let pcnt = (self.len() as f64) / (self.og_len as f64) * 100.0;

		format!("{} ({:.1}%)", self.len(), pcnt)
	}

	/// keep all words that have the given char
	fn contains(&mut self, c: char) {
		self.data.retain(|word| word.contains(c));
	}

	/// remove all words that have the given char
	pub fn gray(&mut self, c: char) {
		self.data.retain(|word| !word.contains(c));
	}

	/// remove all words that have the chars in the str
	pub fn gray_string(&mut self, s: &str) {
		// TODO: do the contains array thingy
		s.chars().for_each(|c| {
			self.data.retain(|word| !word.contains(c));
		});
	}
	
	/// keep all words that have the given char, but not at the specified index
	pub fn yellow(&mut self, c: char, i: usize) {
		if c != ' ' {
			self.contains(c);
			self.data.retain(|word| word.as_bytes()[i] as char != c);
		}
	}

	/// keep words that have the given char at the specified index
	/// letters can be used twice
	pub fn green(&mut self, c: char, i: usize) {
		if c != ' ' {
			self.data.retain(|word| word.as_bytes()[i] as char == c);
		}
	}

	pub fn execute(&mut self, w: Letter) {
		match w {
			Letter::Gray(c) => {
				self.gray(c);
			},
			Letter::Yellow(c, i) => {
				self.yellow(c, i);
			},
			Letter::Green(c, i) => {
				self.green(c, i);
			},
			Letter::Blank(c) => {
				panic!("missing color code for {}\n{:?}", c, w);
			},
			Letter::Empty => {
				panic!("letter missing!");
			}
		}
	}
}