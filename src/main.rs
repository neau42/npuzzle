/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/01 19:30:12 by no                #+#    #+#             */
/*   Updated: 2019/02/05 06:26:25 by no               ###   ########.fr       */
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
		puzzle.actual_dst = 0;
		solver::solve(&mut puzzle);
	}
	else {
		println!("NOT VALID");
	}
}