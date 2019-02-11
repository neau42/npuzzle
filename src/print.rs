use std::collections::HashMap;
use crate::puzzle;
// use crate::puzzle::Puzzle;
use std::thread;
use std::time;
extern crate termion;
use crate::options::Options;


pub fn	print_all(p: & Vec<u16>, close_list: &HashMap<Vec<u16>, (i32, i32, Vec<u16>)>,
prev: &Vec<u16>, final_state: &puzzle::FinalPuzzle, opts: &Options) -> u32 {

	let ref mut end: Vec<Vec<u16>> = Vec::new();
	let mut len: u32 = 0;
	let mut puzzle = p;
	let mut predecessor = prev;

	loop {
		end.push((*puzzle).clone());
		puzzle = predecessor;
		predecessor = match close_list.get(predecessor) {
			Some(x) => &x.2,
			_       => break,
		};
		len += 1;
	}
	let mut elem: Vec<u16>;
	loop {
		elem = match end.pop() {
			Some(x) => x,
			_       => break, 
		};
		if opts.sleep {
			println!("{}[2J{}",27 as char , termion::cursor::Goto(1, 1));
		}
		println!("N-puzzle: ");
		puzzle::print_puzzle(&elem, final_state, opts);
		if opts.sleep {
			thread::sleep(time::Duration::from_millis(200));
		}
	}
	let c_list_len = close_list.len();
	println! ("nb movements: {} ", len);
	println! ("Close list length : {} ", c_list_len);
	len
}
