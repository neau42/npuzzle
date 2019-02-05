/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/01 19:30:12 by no                #+#    #+#             */
/*   Updated: 2019/02/05 17:00:43 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

extern crate colored;
use crate::puzzle::Puzzle;
pub mod parser;
pub mod solver;
pub mod puzzle;
pub mod heuristics;

fn main() {

	let file_name = parser::get_arg();
	let mut puzzle = match parser::get_puzzle(file_name) {
		Ok(t) => t,
		Err(e) => {
			eprintln!("error: {}", e);
			return ;
		}
	};
	if puzzle.is_valid() {
		let final_state: puzzle::FinalPuzzle = puzzle::init_final_stat(puzzle.size as usize);
		if puzzle.is_soluble(&final_state) {
			// let final_state = Puzzle::gen_final_state(puzzle.size as usize);
			puzzle.estimate_dst = heuristics::distance_estimator(&puzzle.taq, &final_state) as i32;
			puzzle.actual_dst = 0;
			solver::solve(&mut puzzle, &final_state);

		} else {
			eprintln!("puzzle not soluble");
		}
	} else {
		eprintln!("not valid puzzle format");
	}
}