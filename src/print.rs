use std::collections::HashSet;
use crate::puzzle;
use crate::puzzle::RefPuzzle;
// use crate::puzzle::Puzzle;
use std::thread;
use std::time;
extern crate termion;
use crate::options::Options;

pub fn	print(close_list: &HashSet<RefPuzzle>,nb_states: usize ,puzzle: &RefPuzzle, final_state: &puzzle::FinalPuzzle, opts: &Options) {

	let ref mut list_final: Vec<Vec<u16>> = Vec::new();
	let mut ref_predecessor;
	let mut predecessor;
	let mut taquin;
	let mut len: u32 = 0;

	// puzzle::print_puzzle(&puzzle.ref_puzzle.borrow().taq , final_state, opts);
	list_final.push(final_state.puzzle.clone());
	let mut puzzle = puzzle.ref_puzzle.borrow();
	loop {
		ref_predecessor = match &puzzle.predecessor {
			Some(ref_next) => ref_next,
			_ 			=> break,
		};
		predecessor = close_list.get(&ref_predecessor).unwrap();
		puzzle = predecessor.ref_puzzle.borrow();
		taquin = &puzzle.taq;
		list_final.push(taquin.clone());
		len += 1;
	// }
	}
	let mut elem: Vec<u16>;
	loop {
		elem = match list_final.pop() {
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
	println! ("###\n# {:?} heuristic", opts.heuristic);
	println! ("# {} movements", len);
	println! ("# {} states selected", close_list.len());
	// println! ("Close list length : {} ", a_l .len());
}

// pub fn	print_all(p: & Vec<u16>, close_list: &HashMap<Vec<u16>, (i32, i32, Vec<u16>)>,
// prev: &Vec<u16>, final_state: &puzzle::FinalPuzzle, opts: &Options) -> u32 {

// 	let ref mut end: Vec<Vec<u16>> = Vec::new();
// 	let mut len: u32 = 0;
// 	let mut puzzle = p;
// 	let mut predecessor = prev;

// 	loop {
// 		end.push((*puzzle).clone());
// 		puzzle = predecessor;
// 		predecessor = match close_list.get(predecessor) {
// 			Some(x) => &x.2,
// 			_       => break,
// 		};
// 		len += 1;
// 	}

// 	let mut len: u32 = 0;
// 	let mut elem: Vec<u16>;
// 	loop {
// 		elem = match end.pop() {
// 			Some(x) => x,
// 			_       => break, 
// 		};
// 		if opts.sleep {
// 			println!("{}[2J{}",27 as char , termion::cursor::Goto(1, 1));
// 		}
// 		println!("N-puzzle: ");
// 		puzzle::print_puzzle(&elem, final_state, opts);
// 		if opts.sleep {
// 			thread::sleep(time::Duration::from_millis(200));
// 		}
// 	}
// 	let c_list_len = close_list.len();
// 	println! ("nb movements: {} ", len);
// 	println! ("Close list length : {} ", c_list_len);
// 	len
// }
