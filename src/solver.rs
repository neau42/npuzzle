use std::thread;
use std::time;
use std::io;
use std::collections::HashMap;

use crate::puzzle::Puzzle;


pub fn update_open_list(puzzle: &mut Puzzle, close_list: &mut HashMap<Puzzle, i32>, open_list: &mut HashMap<Puzzle, i32>, final_state: & Puzzle) {
	static MOVE_FUNCTIONS: &[ fn(&Puzzle, usize) -> (Result<Puzzle, io::Error>); 4]  = &[Puzzle::move_down, Puzzle::move_up, Puzzle::move_left, Puzzle::move_right];
	let zero_pos = puzzle.get_pos_of_value(0) as usize;

	for function in MOVE_FUNCTIONS {
			match function(&puzzle, zero_pos) {
			Ok(mut a) => if close_list.contains_key(&a) {
				a.distance_estimator(&final_state);
				a.actual_len = puzzle.actual_len + 1;
				if open_list.contains_key(&a) {
					if a.esimate_dst + a.actual_len >= *open_list.get(&a).unwrap() {
						continue ;
					}
				}
				open_list.insert(a.copy(), a.esimate_dst + a.actual_len);
			}
			Err(_) => (),
		}
	// open_list.sort_by(|a, b| ((b.esimate_dst + b.actual_len).cmp(&(a.esimate_dst + a.actual_len))));
	}
}

pub fn solve(close_list: &mut HashMap<Puzzle, i32>, open_list: &mut HashMap<Puzzle, i32>, puzzle: &mut Puzzle) {
	let final_state = Puzzle::gen_final_state(puzzle.size as usize);
	// for _i in 0..3000 {
		loop {

		// let ref mut puzzle = open_list.pop().unwrap();
		// let ref mut puzzle = open_list.get(&1);

		println!("current puzzle: ");
		puzzle.print();
		if puzzle.esimate_dst == 0 {
			println!("SUCCESS -_-");
			puzzle.print();
			break ;
		}
		// open_list.
		update_open_list(puzzle, open_list, close_list, &final_state);
		for e in open_list.iter() {
			e.0.print();
		}
		// close_list.push(puzzle.copy());
		close_list.insert(puzzle.copy(), puzzle.esimate_dst);
		thread::sleep(time::Duration::from_millis(350));
	}
	println!("open list LEN {}", open_list.len());
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
// 				if a.esimate_dst + a.actual_len < open_list[tst_pos].esimate_dst + open_list[tst_pos].actual_len {
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
		 		
// 				// //  dbg!(&open_list[tst_pos].esimate_dst);
// 				// // dbg!(a.distance_estimator(&final_state));
// 		 		// dbg!(&open_list[tst_pos].actual_len);
// 				// dbg!(puzzle.actual_len + 1);

// 				// }
// 				open_list.push(a);
// 			}
// 			Err(_) => (),
// 		}
// 	open_list.sort_by(|a, b| ((b.esimate_dst + b.actual_len).cmp(&(a.esimate_dst + a.actual_len))));
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
// 		if puzzle.esimate_dst == 0 {
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
