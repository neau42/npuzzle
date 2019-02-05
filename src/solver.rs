use std::thread;
use std::time;
use std::io;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;

// use crate::parser;
use crate::puzzle;
use crate::puzzle::Puzzle;
use crate::puzzle::PuzzleRes;

fn update_open_list(puzzle: & Puzzle, open_list: &mut  BinaryHeap<PuzzleRes> , final_state: & Puzzle, all_list: &mut HashSet<Vec<u8>>) {
	static MOVE_FUNCTIONS: &[ fn(&Vec<u8>, usize, usize) -> Result<Vec<u8>, io::Error>; 4] =
	&[puzzle::move_up, puzzle::move_down, puzzle::move_left, puzzle::move_right];
	let zero_pos = puzzle.get_pos_of_value(0) as usize;

	for function in MOVE_FUNCTIONS {
		match function(&puzzle.taq, zero_pos, puzzle.size as usize) {
			Ok(taquin) => {
				let dst = puzzle::distance_estimator(&taquin, final_state);
				if !all_list.contains(&taquin) {
					open_list.push(PuzzleRes{taq: taquin.clone(),estimate_dst: dst, actual_dst: puzzle.actual_dst + 1,predecessor: puzzle.taq.clone()} );
					all_list.insert(taquin.clone());
				}
			}
			_ =>  (),
		}
	}
}

fn print_all(puzzle: &mut Puzzle, close_list: &HashMap<Vec<u8>, (i32, i32, Vec<u8>)>, size: i32, mut predecessor: Vec<u8>) -> u32 {
	let ref mut end: Vec<Vec<u8>> = Vec::new();
	let mut i: u32 = 0;

	loop {
		end.push(puzzle.taq.clone());
		puzzle.taq = predecessor.clone();
		predecessor = match close_list.get(&predecessor) {
			Some(x) => x.2.clone(),
			_ => break,
		};
		i += 1;
	}
	loop {
		let p = match end.pop() {
			Some(x) => x,
			_ => break, 
		};
		println!("N-puzzle: ");
		puzzle::print_puzzle(&p, size as usize);
			// thread::sleep(time::Duration::from_millis(250));
	}
	i
}

pub fn solve(puzzle: &mut Puzzle) {
	let final_state = Puzzle::gen_final_state(puzzle.size as usize);
	let mut close_list: HashMap<Vec<u8>, (i32, i32, Vec<u8>)> = HashMap::new();
	let mut open_list: BinaryHeap<PuzzleRes>  = BinaryHeap::new();
	let mut all_list: HashSet<Vec<u8>> = HashSet::new();

	let ref mut o_list = open_list;
	let ref mut c_list = close_list;
	let ref mut a_list = all_list;

	let ref mut predecessor: Vec<u8> = Vec::new();
	let mut next: PuzzleRes;

	a_list.insert(puzzle.taq.clone());

	let mut nb_state: u32 = 0;
	let mut max_state: usize = 0;

	while puzzle::distance_estimator(&puzzle.taq, &final_state) != 0 {

		c_list.insert(puzzle.taq.clone(), (puzzle.estimate_dst, puzzle.actual_dst, predecessor.clone()));
		update_open_list(puzzle, o_list, &final_state, a_list);

		next = o_list.pop().unwrap();
		*predecessor = next.predecessor.clone();
		puzzle.taq = next.taq.clone();
		puzzle.actual_dst = next.actual_dst;
		nb_state += 1;
		if max_state < o_list.len() {
			max_state = o_list.len();
		}
		
		// thread::sleep(time::Duration::from_millis(250));
	}
	println! ("Success: -_-| ");
	let len = print_all(puzzle, c_list, final_state.size, predecessor.clone());
	println! ("len: {} ", len);
	println! ("Open_list.len() : {} ", o_list.len());
	println! ("Close_list.len() : {} ", c_list.len());
	println! ("All_list.len() : {}", a_list.len());
	// println! ("nb states: {} ", nb_state); // == c_list.len()
	// println! ("max rep: {} ", max_state); // == o_list.len()
}