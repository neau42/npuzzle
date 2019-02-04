use std::thread;
use std::time;
use std::io;
use std::collections::HashMap;

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
				// if !open_list.contains_key(&taquin) && (!close_list.contains_key(&taquin) || *close_list.get(&taquin).unwrap() >= dst) {
				if !open_list.contains_key(&taquin) && !close_list.contains_key(&taquin) {
				let dst = puzzle::distance_estimator(&taquin, final_state);
					open_list.insert(taquin.clone(),( dst, puzzle.actual_dst + 1, puzzle.taq.clone()));
				}
			}
			_ =>  (),
		}
	}
}

pub fn find_better_in_open_list(open_list: &mut HashMap<Vec<u8>, (i32, i32, Vec<u8>)>, final_state: & Puzzle, predecessor: &mut Vec<u8>) -> (Vec<u8>, i32) {

	let mut tmp_dst: i32 = std::i32::MAX;
	let mut tmp_vec: Vec<u8> = Vec::new();
	let mut tmp_actual_dst: i32 = std::i32::MAX;

	for (vec, (estimate_dst, actual_dst, prev)) in open_list.iter() {
		// println!("OPEN LIST: :: {:?}::{}",vec, estimate_dst);
		// puzzle::print_puzzle(&vec, final_state.size as usize);
		if *estimate_dst < tmp_dst {
			tmp_dst = *estimate_dst;
			tmp_vec = vec.clone();
			tmp_actual_dst = *actual_dst;
			*predecessor = (*prev).clone();
		}
	}
	(tmp_vec, tmp_actual_dst)
}

fn print_all(puzzle: &mut Puzzle, close_list: &mut HashMap<Vec<u8>, (i32, i32, Vec<u8>)>, size: i32, mut predecessor: Vec<u8>) -> u32 {
	let ref mut end: Vec<Vec<u8>> = Vec::new();

	let mut i: u32 = 0;
	loop {
		// println!("N-puzzle: ");
		// puzzle::print_puzzle(&puzzle.taq, size as usize);
		end.push(puzzle.taq.clone());
		puzzle.taq = predecessor.clone();
		let t = match close_list.get(&predecessor) {
			Some(x) => x.2.clone(),
			_ => break,
		};
		predecessor = t;
		i += 1;
			// thread::sleep(time::Duration::from_millis(250));
	}
	loop {
		let p = match end.pop() {
			Some(x) => x,
			_ => break, 
		};
		println!("N-puzzle: ");
		puzzle::print_puzzle(&p, size as usize);
	}
	i
}

pub fn solve(puzzle: &mut Puzzle) {
	let final_state = Puzzle::gen_final_state(puzzle.size as usize);
	let mut close_list: HashMap<Vec<u8>, (i32, i32, Vec<u8>)> = HashMap::new();
	let mut open_list: HashMap<Vec<u8>, (i32, i32, Vec<u8>)>  = HashMap::new();
	let ref mut o_list = open_list;
	let ref mut c_list = close_list;
	let ref mut predecessor: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

	o_list.insert(puzzle.taq.clone(), (puzzle.estimate_dst, 0, predecessor.clone()));

	while puzzle::distance_estimator(&puzzle.taq, &final_state) != 0 {
		c_list.insert(puzzle.taq.clone(), o_list.remove(&puzzle.taq).unwrap());
		update_open_list(puzzle, c_list, o_list, &final_state);
		let t = find_better_in_open_list(o_list, &final_state, predecessor);
		puzzle.taq = t.0;
		puzzle.actual_dst = t.1;
		// println!("[tst]: predecessor: {:?}", predecessor);
	}
	println! ("Success: -_-| ");
	let len = print_all(puzzle, c_list, final_state.size, predecessor.clone());

	println! ("LEN: {} ", len);
	println! ("Open_list.len() : {} ", o_list.len());
	println! ("Close_list.len() : {} ", c_list.len());

	// println!("CLOSE LIST LEN : :: {}", c_list.len());
	// for (vec, (estimate_dst, actual_dst, prev)) in c_list.iter() {
	// 		println!("CLOSE LIST: :: {} / {} prev: {:?}", estimate_dst, actual_dst, prev);
	// 		puzzle::print_puzzle(&vec, final_state.size as usize);
	// 		println!("PREV: ");
	// 		puzzle::print_puzzle(&prev, final_state.size as usize);
	// }
	// println!("OPEN LIST LEN : :: {}", o_list.len());
	// for (vec, (estimate_dst, actual_dst, prev)) in o_list.iter() {
	// 	println!("OPEN LIST: :: {} / {} prev: {:?}", estimate_dst, actual_dst, prev);
	// 	puzzle::print_puzzle(&vec, final_state.size as usize);
	// 	println!("PREV: ");
	// 	puzzle::print_puzzle(&prev, final_state.size as usize);

	// }
	// println!("-----------------\n\n");

			// println!("puzzle: ");
			// puzzle::print_puzzle(&puzzle.taq, final_state.size as usize);
			// let t = close_list.get(&predecessor).unwrap();
			// puzzle.taq = predecessor;
			// predecessor = t.2.clone();

			// println!("puzzle: ");
			// puzzle::print_puzzle(&puzzle.taq, final_state.size as usize);
			// let t = close_list.get(&predecessor).unwrap();
			// puzzle.taq = predecessor;
			// predecessor = t.2.clone();

			// println!("puzzle: ");
			// puzzle::print_puzzle(&puzzle.taq, final_state.size as usize);
			// let t = close_list.get(&predecessor).unwrap();
			// puzzle.taq = predecessor;
			// predecessor = t.2.clone();

			// puzzle::print_puzzle(&predecessor, final_state.size as usize);


	// 		puzzle.taq = close_list.get(&predecessor).unwrap().2.clone();
	// 		println!("--");
	// 		thread::sleep(time::Duration::from_millis(350));

	// }



	
}




// fn update_open_list(puzzle: &mut Puzzle, open_list: &mut Vec<Puzzle>, close_list: &mut Vec<Puzzle>, final_state: & Puzzle) {
// 	static MOVE_FUNCTIONS: &[ fn(&Puzzle, usize) -> (Result<Puzzle, io::Error>); 4]  = &[Puzzle::move_down, Puzzle::move_up, Puzzle::move_left, Puzzle::move_right];
// 	let zero_pos = puzzle.get_pos_of_value(0) as usize;

// 	for function in MOVE_FUNCTIONS {
// 			match function(&puzzle, zero_pos) {
// 			Ok(mut a) => if !close_list.contains(&a) {//&& !open_list.contains(&a) {
// 				a.distance_estimator(&final_state);
// 				a.actual_dst = puzzle.actual_dst + 1;
// 				if open_list.contains(&a) {
// 				let tst_pos = open_list.iter().position(|r| *r == a).unwrap();
// 				if a.estimate_dst + a.actual_dst < open_list[tst_pos].estimate_dst + open_list[tst_pos].actual_dst {
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
// 		 		// dbg!(&open_list[tst_pos].actual_dst);
// 				// dbg!(puzzle.actual_dst + 1);

// 				// }
// 				open_list.push(a);
// 			}
// 			Err(_) => (),
// 		}
// 	open_list.sort_by(|a, b| ((b.estimate_dst + b.actual_dst).cmp(&(a.estimate_dst + a.actual_dst))));
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
