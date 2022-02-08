use super::*;

pub fn code() {
	let mut words = List::new();
	dbg!(&words.len());
	
	words.gray_string("atpifucsy");
	dbg!(&words.len());

	words.yellow('l', 0);
	words.yellow('l', 2);
	words.green('e', 3);
	words.green('r', 4);
	words.green('l', 1);
	dbg!(&words.len());

	dbg!(&words.data);
}