/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/01 19:30:12 by no                #+#    #+#             */
/*   Updated: 2019/02/01 19:57:48 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

extern crate colored;
use colored::*;
use crate::puzzle::Puzzle;
use std::io;
pub mod parser;
pub mod puzzle;
// use std::collections::Vec;


fn create_open_list(puzzle: &mut Puzzle, open_list: &mut Vec<Puzzle>, close_list: &mut Vec<Puzzle>, final_state: & Puzzle) {
	// let functions: Vec< for<'r> fn(&'r Puzzle) -> (Result<Puzzle, io::Error>)> = vec![Puzzle::move_down, Puzzle::move_up, Puzzle::move_left, Puzzle::move_right];
	static MOVE_FUNCTIONS: &[ fn(&Puzzle, usize) -> (Result<Puzzle, io::Error>); 4]  = &[Puzzle::move_down, Puzzle::move_up, Puzzle::move_left, Puzzle::move_right];
	let zero_pos = puzzle.get_pos_of_value(0) as usize;

	for function in MOVE_FUNCTIONS {
			match function(&puzzle, zero_pos) {
			Ok(mut a) => if !close_list.contains(&a) && !open_list.contains(&a) {
				// if open_list.contains(&a) {
				// 	println!("open_list CONTAIN!");
				// let tst_pos = open_list.iter().position(|r| *r == a).unwrap();
		 		
				// //  dbg!(&open_list[tst_pos].esimate_dst);
				// // dbg!(a.distance_estimator(&final_state));
		 		// dbg!(&open_list[tst_pos].actual_len);
				// dbg!(puzzle.actual_len + 1);

				// }
				a.esimate_dst = a.distance_estimator(&final_state) as i32;
				a.actual_len = puzzle.actual_len + 1;
				open_list.push(a);
			}
			Err(_) => (),
		}
	open_list.sort_by(|a, b| ((b.esimate_dst + b.actual_len).cmp(&(a.esimate_dst + a.actual_len))));
	}
}

fn solve(close_list: &mut Vec<Puzzle>, open_list: &mut Vec<Puzzle>, size: i32) {
	let final_state = Puzzle::gen_final_state(size as usize);
	// for _i in 0..3000 {
		loop {
		let ref mut puzzle = open_list.pop().unwrap();
		println!("current puzzle: ");
		puzzle.print();
		if puzzle.esimate_dst == 0 {
			println!("SUCCESS -_-");
			puzzle.print();
			break ;
		}
		// println!("{}", "close list: ".green());
		// for e in close_list.iter() {
		// 	e.print();
		// }
		create_open_list(puzzle, open_list, close_list, &final_state);
		close_list.push(puzzle.copy());
		// println!("{}", "open list: ".green());
		// for e in open_list.iter() {
		// 	e.print();
		// }
		// println!("---------------------------- end");
		// thread::sleep(time::Duration::from_millis(350));
	}
	println!("open list LEN {}", open_list.len());
}

fn main() {
	let ref mut close_list: Vec<Puzzle> = Vec::new();
	let ref mut open_list: Vec<Puzzle> = Vec::new();

	let file_name = parser::get_arg();
	let ref mut puzzle = match parser::get_puzzle(file_name) {
		Ok(t) => t,
		Err(e) => {
			eprintln!("error: {}",e);
			return ;
		}
	};
	if puzzle.is_valid() && puzzle.is_soluble() {
		println!("VALID and Soluble!");
		puzzle.print();
		let final_state = Puzzle::gen_final_state(puzzle.size as usize);
		puzzle.esimate_dst = puzzle.distance_estimator(&final_state) as i32;
		puzzle.actual_len = 0;
		// create_open_list(puzzle, open_list, close_list);
		open_list.push(puzzle.copy());
		// close_list.push(puzzle.copy());
		solve(close_list, open_list, puzzle.size);
	}
	else {
		println!("NOT VALID");
	}
}


			// 	match self.move_down() {
			// 	Ok(a) => match close_list.contains(&a) {
			// 		true  => (),
			// 		false => open_list.push(a),
			// 		}
			// 	Err(_) => (),
			// }
			// match self.move_up() {
			// 	Ok(a) => match close_list.contains(&a) {
			// 		true  => (),
			// 		false => open_list.push(a),
			// 		}
			// 	Err(_) => (),
			// }
			// match self.move_left() {
			// 	Ok(a) => match close_list.contains(&a) {
			// 		true  => (),
			// 		false => open_list.push(a),
			// 		}
			// 	Err(_) => (),
			// }
			// match self.move_right() {
			// 	Ok(a) => match close_list.contains(&a) {
			// 		true  => (),
			// 		false => open_list.push(a),
			// 		}
			// 	Err(_) => (),
			// }

