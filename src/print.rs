use std::collections::HashMap;
use std::collections::HashSet;
use crate::puzzle;
use crate::puzzle::RefPuzzle;
// use crate::puzzle::Puzzle;
use std::thread;
use std::time;
extern crate termion;
use crate::options::Options;

pub fn	print(close_list: &HashSet<RefPuzzle>, mut puzzle: &RefPuzzle, final_state: &puzzle::FinalPuzzle, opts: &Options) {

	// let mut next = &close_list.get(puzzle).unwrap().ref_puzzle.borrow().predecessor;

	// let mut nx = match next {
	// 	Some(x) => x,
	// 	_	=> return,
	// };
	// println!("LAST: ");
	// puzzle::print_puzzle(&puzzle.ref_puzzle.borrow().taq , final_state, opts);

	puzzle::print_puzzle(&puzzle.ref_puzzle.borrow().taq , final_state, opts);
	let mut test1 = puzzle.ref_puzzle.borrow();
	loop {
		let mut next = match &test1.predecessor {
			Some(ref_next) => ref_next,
			_ 			=> return,
		};
		let new1 = close_list.get(&next).unwrap();
		test1 = new1.ref_puzzle.borrow();
		let a = &test1.taq;
		puzzle::print_puzzle(&a , final_state, opts);
		println!("~~");
	}
	// dbg!(a);

	// println!("{}");
	

	// puzzle::print_puzzle(&next , final_state, opts);
	// puzzle = close_list.get(&nx).unwrap();
}

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
