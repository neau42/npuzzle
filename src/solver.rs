/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   solver.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/05 10:34:10 by no                #+#    #+#             */
/*   Updated: 2019/02/11 11:31:16 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::time::Instant;
use crate::heuristics;
use crate::puzzle;
use crate::print;
use crate::puzzle::Puzzle;
use crate::puzzle::PuzzleRes;
use crate::options::Options;

fn update_open_list(puzzle: & Puzzle, open_list: &mut  BinaryHeap<PuzzleRes> , final_state: & puzzle::FinalPuzzle, all_list: &mut HashSet<Vec<u16>>, opts: &Options) {
	let zero_pos = puzzle.get_pos_of_value(0) as usize;
	static MOVE_FUNCTIONS: &[ fn(&Vec<u16>, usize, usize) -> Result<Vec<u16>, io::Error>; 4] =
		&[puzzle::move_up, puzzle::move_down, puzzle::move_left, puzzle::move_right];
	let mut dst;

	for function in MOVE_FUNCTIONS {
		match function(&puzzle.taq, zero_pos, puzzle.size as usize) {
			Ok(taquin) => {
				if !all_list.contains(&taquin) {
					dst = heuristics::distance_estimator(&taquin, final_state, opts);
					open_list.push(PuzzleRes{taq: taquin.clone(),estimate_dst: dst, actual_dst: puzzle.actual_dst + 1,predecessor: puzzle.taq.clone()} );
					all_list.insert(taquin.clone());
				}
			}
			_ =>  (),
		}
	}
}

pub fn solve(puzzle: &mut Puzzle, final_state: &puzzle::FinalPuzzle, opts: Options) {
	let mut close_list: HashMap<Vec<u16>, (i32, i32, Vec<u16>)> = HashMap::new();
	let mut open_list: BinaryHeap<PuzzleRes>  = BinaryHeap::new();
	let mut all_list: HashSet<Vec<u16>> = HashSet::new();

	let ref mut c_list = close_list;
	let ref mut o_list = open_list;
	let ref mut a_list = all_list;

	puzzle.estimate_dst = heuristics::distance_estimator(&puzzle.taq, &final_state, &opts) as i32;
	puzzle.actual_dst = 0;

	let mut next = PuzzleRes{taq: Vec::new(), estimate_dst: 0 , actual_dst:0 , predecessor:Vec::new() };

	a_list.insert(puzzle.taq.clone());

	let start = Instant::now();
	while puzzle.estimate_dst != 0 {
		c_list.insert(puzzle.taq.clone(), (puzzle.estimate_dst, puzzle.actual_dst, next.predecessor.clone()));
		update_open_list(puzzle, o_list, final_state, a_list, &opts);
		next = o_list.pop().unwrap();
		puzzle.taq = next.taq;
		puzzle.actual_dst = match opts.greedy {
			false =>	next.actual_dst,
			true =>		0,
		};
		puzzle.estimate_dst = next.estimate_dst;
	}
	let elapsed = start.elapsed();
	println! ("Success :)");
	print::print_all(puzzle, c_list, final_state.size, next.predecessor.clone(), final_state, &opts);
	println!("Time  : {:?}", elapsed);
}