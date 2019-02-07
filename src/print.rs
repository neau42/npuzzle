use std::collections::HashMap;
use crate::puzzle;
use crate::puzzle::Puzzle;
use std::thread;
use std::time;
extern crate termion;


pub fn	print_all(puzzle: &mut Puzzle, close_list: &HashMap<Vec<u8>, (i32, i32, Vec<u8>)>,
		size: i32, mut predecessor: Vec<u8>, final_state: &puzzle::FinalPuzzle) -> u32 {

	let ref mut end: Vec<Vec<u8>> = Vec::new();
	let mut len: u32 = 0;

	loop {
		end.push(puzzle.taq.clone());
		puzzle.taq = predecessor.clone();
		predecessor = match close_list.get(&predecessor) {
			Some(x) => x.2.clone(),
			_ => break,
		};
		len += 1;
	}
	let mut elem: Vec<u8>;
	loop {
		elem = match end.pop() {
			Some(x) => x,
			_ => break, 
		};
		println!("{}[2J{}",27 as char , termion::cursor::Goto(1, 1));
		println!("N-puzzle: ");
		puzzle::print_puzzle(&elem, size as usize, final_state);
		thread::sleep(time::Duration::from_millis(200));
	}
	let c_list_len = close_list.len();
	println! ("nb movements: {} ", len);
	println! ("Close list length : {} ", c_list_len);
	len
}
