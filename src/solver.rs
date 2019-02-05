use std::thread;
use std::time;
use std::io;
use std::collections::HashMap;
use std::collections::HashSet;

// use crate::parser;
use crate::puzzle;
use crate::puzzle::Puzzle;


pub fn update_open_list(puzzle: & Puzzle, close_list: &mut HashMap<Vec<u8>, (i32, i32, Vec<u8>)>, open_list: & mut HashMap<Vec<u8>, (i32, i32, Vec<u8>)>, final_state: & Puzzle) {
	static MOVE_FUNCTIONS: &[ fn(&Vec<u8>, usize, usize) -> Result<Vec<u8>, io::Error>; 4] =
	&[puzzle::move_up, puzzle::move_down, puzzle::move_left, puzzle::move_right];
	let zero_pos = puzzle.get_pos_of_value(0) as usize;

	for function in MOVE_FUNCTIONS {
		match function(&puzzle.taq, zero_pos, puzzle.size as usize) {
			Ok(taquin) => {
				let dst = puzzle::distance_estimator(&taquin, final_state);
				// if !open_list.contains_key(&taquin) && (!close_list.contains_key(&taquin) || close_list.get(&taquin).unwrap().0 + close_list.get(&taquin).unwrap().1 >= puzzle.actual_dst + 1 +dst) {
				if !open_list.contains_key(&taquin) && !close_list.contains_key(&taquin) {
					open_list.insert(taquin.clone(),( dst, puzzle.actual_dst + 1, puzzle.taq.clone()));
				}
			}
			_ =>  (),
		}
	}
}

pub fn find_better_in_open_list(open_list: &HashMap<Vec<u8>, (i32, i32, Vec<u8>)>, predecessor: &mut Vec<u8>) -> (Vec<u8>, i32) {

	let mut tmp_dst: i32 = std::i32::MAX / 2;
	let mut tmp_vec: Vec<u8> = Vec::new();
	let mut tmp_actual_dst: i32 = std::i32::MAX / 2;
	let mut tmp_sum = tmp_dst + tmp_actual_dst;

	for (vec, (estimate_dst, actual_dst, prev)) in open_list {
		if estimate_dst + actual_dst == tmp_sum && *actual_dst > tmp_actual_dst {
				tmp_dst = *estimate_dst;
				tmp_vec = vec.clone();
				tmp_actual_dst = *actual_dst;
				*predecessor = (prev).clone();
				tmp_sum = tmp_dst + tmp_actual_dst;
			}
		else if estimate_dst + actual_dst < tmp_sum {
			tmp_dst = *estimate_dst;
			tmp_vec = vec.clone();
			tmp_actual_dst = *actual_dst;
			*predecessor = (prev).clone();
			tmp_sum = tmp_dst + tmp_actual_dst;
		}
	}
	(tmp_vec, tmp_actual_dst)
}

fn print_all(puzzle: &mut Puzzle, close_list: &mut HashMap<Vec<u8>, (i32, i32, Vec<u8>)>, size: i32, mut predecessor: Vec<u8>) -> u32 {
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
	let mut open_list: HashMap<Vec<u8>, (i32, i32, Vec<u8>)>  = HashMap::new();
	// let mut all_list: HashSet<Vec<u8>> = HashSet::new();
	let ref mut o_list = open_list;
	let ref mut c_list = close_list;
	// let ref mut a_list = all_list;
	let ref mut predecessor: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

	o_list.insert(puzzle.taq.clone(), (puzzle.estimate_dst, 0, predecessor.clone()));
	// a_list.insert(puzzle.taq.clone());

	while puzzle::distance_estimator(&puzzle.taq, &final_state) != 0 {
		c_list.insert(puzzle.taq.clone(), o_list.remove(&puzzle.taq).unwrap());
		update_open_list(puzzle, c_list, o_list, &final_state);
		let t = find_better_in_open_list(o_list, predecessor);
		puzzle.taq = t.0;
		puzzle.actual_dst = t.1;
	}
	println! ("Success: -_-| ");
	let len = print_all(puzzle, c_list, final_state.size, predecessor.clone());

	println! ("LEN: {} ", len);
	println! ("Open_list.len() : {} ", o_list.len());
	println! ("Close_list.len() : {} ", c_list.len());
}