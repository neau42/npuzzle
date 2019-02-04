/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/01 19:30:12 by no                #+#    #+#             */
/*   Updated: 2019/02/04 01:29:21 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// use std::collections::HashMap;
extern crate colored;
// use colored::*;
// use std::thread;
// use std::time;
use crate::puzzle::Puzzle;
// use std::io;
pub mod parser;
pub mod solver;
pub mod puzzle;

fn main() {
	// let ref mut close_list: HashMap<puzzle::Puzzle, i32> = HashMap::new();
	// let ref mut open_list: HashMap<puzzle::Puzzle, i32> = HashMap::new();
	// let ref mut close_list: Vec<Puzzle> = Vec::new();
	// let ref mut open_list: Vec<Puzzle> = Vec::new();
	
	

	let file_name = parser::get_arg();
	let mut puzzle = match parser::get_puzzle(file_name) {
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
		puzzle.estimate_dst = puzzle::distance_estimator(&puzzle.taq, &final_state) as i32;
		puzzle.actual_len = 0;
		// open_list.insert(puzzle, i);


		// for val in open_list.keys() {
		// 	println!("{:?}", val);
		// }
		// open_list.push(puzzle.copy());
		// solve(close_list, open_list, puzzle.size);
		solver::solve(&mut puzzle);
	}
	else {
		println!("NOT VALID");
	}
}