// use std::thread;
// use std::time;
use std::io;
use std::collections::HashMap;

// use crate::parser;
use crate::puzzle;
use crate::puzzle::Puzzle;

pub fn update_open_list(puzzle: &mut Puzzle, close_list: &mut HashMap<Vec<u8>, i32>, open_list: & mut HashMap<Vec<u8>, i32>, final_state: & Puzzle) {
	static MOVE_FUNCTIONS: &[ fn(&Vec<u8>, usize, usize) -> Result<Vec<u8>, io::Error>; 4] =
	&[puzzle::move_up, puzzle::move_down, puzzle::move_left, puzzle::move_right];
	let zero_pos = puzzle.get_pos_of_value(0) as usize;

	for function in MOVE_FUNCTIONS {
		match function(&puzzle.taq, zero_pos, puzzle.size as usize) {
			Ok(taquin) => {
				let dst = puzzle::distance_estimator(&taquin, final_state);
				// if !open_list.contains_key(&taquin) && (!close_list.contains_key(&taquin) || *close_list.get(&taquin).unwrap() >= dst) {
				if !open_list.contains_key(&taquin) && !close_list.contains_key(&taquin) {
					open_list.insert(taquin.clone(), dst);
				}
			}
			_ =>  (),
		}
	}
}

pub fn find_better_in_open_list(open_list: &mut HashMap<Vec<u8>, i32>, final_state: & Puzzle) -> Vec<u8> {

	let mut tmp_dst: i32 = std::i32::MAX;
	let mut tmp_vec: Vec<u8> = Vec::new();

	for (vec, dst) in open_list.iter() {
		// println!("OPEN LIST: :: {:?}::{}",vec, dst);
		// puzzle::print_puzzle(&vec, final_state.size as usize);
		if *dst < tmp_dst {
			tmp_dst = *dst;
			tmp_vec = vec.clone();
		}
	}
	
	// println!("---------------");
	// println!("BETTER:: {:?}::{}",tmp_vec, tmp_dst);
	// puzzle::print_puzzle(&tmp_vec, final_state.size as usize);
	// println!("---------------");

	tmp_vec
	// Puzzle::gen_final_state(final_state.size as usize).taq
}

pub fn solve(puzzle: &mut Puzzle) {
	let final_state = Puzzle::gen_final_state(puzzle.size as usize);
	let mut close_list: HashMap<Vec<u8>, i32> = HashMap::new();
	let mut open_list: HashMap<Vec<u8>, i32>  = HashMap::new();
	let ref mut o_list = open_list;
	let ref mut c_list = close_list;
	let mut dst;

	o_list.insert(puzzle.taq.clone(), puzzle.estimate_dst);

	// for _i in 0..5 {
	loop {
		dst = puzzle::distance_estimator(&puzzle.taq, &final_state);
		if dst == 0 {
			println! ("Success: -_-|");
			break ;
		}
		c_list.insert(puzzle.taq.clone(), dst);
		o_list.remove(&puzzle.taq);
		update_open_list(puzzle, c_list, o_list, &final_state);
		puzzle.taq = find_better_in_open_list(o_list, &final_state);
	}

	println!("CLOSE LIST LEN : :: {}", c_list.len());
	for (ve, dst) in c_list.iter() {
		println!("CLOSE LIST: :: {}", dst);
		puzzle::print_puzzle(&ve, final_state.size as usize);
	}
	println!("OPEN LIST LEN : :: {}", o_list.len());
	for (ve, dst) in o_list.iter() {
		println!("OPEN LIST: :: {}", dst);
		puzzle::print_puzzle(&ve, final_state.size as usize);
	}

	
}




// fn update_open_list(puzzle: &mut Puzzle, open_list: &mut Vec<Puzzle>, close_list: &mut Vec<Puzzle>, final_state: & Puzzle) {
// 	static MOVE_FUNCTIONS: &[ fn(&Puzzle, usize) -> (Result<Puzzle, io::Error>); 4]  = &[Puzzle::move_down, Puzzle::move_up, Puzzle::move_left, Puzzle::move_right];
// 	let zero_pos = puzzle.get_pos_of_value(0) as usize;

// 	for function in MOVE_FUNCTIONS {
// 			match function(&puzzle, zero_pos) {
// 			Ok(mut a) => if !close_list.contains(&a) {//&& !open_list.contains(&a) {
// 				a.distance_estimator(&final_state);
// 				a.actual_len = puzzle.actual_len + 1;
// 				if open_list.contains(&a) {
// 				let tst_pos = open_list.iter().position(|r| *r == a).unwrap();
// 				if a.estimate_dst + a.actual_len < open_list[tst_pos].estimate_dst + open_list[tst_pos].actual_len {
// 					open_list.remove(tst_pos);
// 				}
// 				else {
// 					continue ;
// 				}
// 				// println!("open_list CONTAIN!");
// 				// println!("in Position: {}\nnew:", tst_pos);
// 				// a.print();
// 				// println!("in open_list:");
// 				// open_list[tst_pos].print();
// 				}
		 		
// 				// //  dbg!(&open_list[tst_pos].estimate_dst);
// 				// // dbg!(a.distance_estimator(&final_state));
// 		 		// dbg!(&open_list[tst_pos].actual_len);
// 				// dbg!(puzzle.actual_len + 1);

// 				// }
// 				open_list.push(a);
// 			}
// 			Err(_) => (),
// 		}
// 	open_list.sort_by(|a, b| ((b.estimate_dst + b.actual_len).cmp(&(a.estimate_dst + a.actual_len))));
// 	}
// }


// fn solve(close_list: &mut Vec<Puzzle>, open_list: &mut Vec<Puzzle>, size: i32) {
// // fn solve(close_list: &mut HashMap<&mut puzzle::Puzzle, i32>, open_list: &mut HashMap<&mut puzzle::Puzzle, i32>, size: i32) {
// 	let final_state = Puzzle::gen_final_state(size as usize);
// 	// for _i in 0..3000 {
// 		loop {
// 		let ref mut puzzle = open_list.pop().unwrap();
// 		// let ref mut puzzle = open_list.get(&1);

// 		// println!("current puzzle: ");
// 		// puzzle.print();
// 		if puzzle.estimate_dst == 0 {
// 			println!("SUCCESS -_-");
// 			puzzle.print();
// 			break ;
// 		}
// 		// println!("{}", "close list: ".green());
// 		// for e in close_list.iter() {
// 		// 	e.print();
// 		// }
// 		update_open_list(puzzle, open_list, close_list, &final_state);
// 		close_list.push(puzzle.copy());
// 		// println!("{}", "open list: ".green());
// 		// for e in open_list.iter() {
// 		// 	e.print();
// 		// }
// 		// println!("---------------------------- end");
// 		// thread::sleep(time::Duration::from_millis(350));
// 	}
// 	println!("open list LEN {}", open_list.len());
// }
