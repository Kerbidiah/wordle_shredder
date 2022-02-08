use super::*;

pub fn code() {
	let mut words = List::new();
	dbg!(words.len());
	
	// started w/ inter
	words.gray('i');
	words.gray('n');
	words.yellow('t', 2);
	words.yellow('e', 3);
	words.gray('r');	
	dbg!(&words.len());
	dbg!(&words.data);

	
	// guess table
	words.green('t', 0);
	words.green('e', 4);
	words.gray('a');
	words.gray('b');
	words.gray('l');
	dbg!(&words.len());
	dbg!(&words.data);


	// guess thyme
	words.green('h', 1);
	words.gray('y');
	words.gray('m');
	dbg!(&words.len());
	dbg!(&words.data);

	
	// guess those
	words.green('o', 2);
	words.green('s', 3);
	// CORRECT

	dbg!(&words.data);
}