use super::*;

pub fn code() {
	let mut words = List::new();
	dbg!(&words.len());
	
	// started w/ inter
	words.yellow('i', 0);
	words.gray_string("nter");
	dbg!(&words.len());

	// guess pubis
	words.gray_string("ub");
	words.yellow('p', 0);
	words.yellow('i', 3);
	words.yellow('s', 4);
	dbg!(&words.len());

	// guess skimp
	words.gray_string("km");
	words.green('s', 0);
	words.green('s', 0);
	words.yellow('p', 4);
	dbg!(&words.len());

	// guess spill
	words.green('p', 1);
	words.gray('l');

	dbg!(&words.len());
	// guess spicy
	// correct!

	dbg!(&words.data);
}