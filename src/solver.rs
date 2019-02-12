/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   solver.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/05 10:34:10 by no                #+#    #+#             */
/*   Updated: 2019/02/11 19:09:04 by no               ###   ########.fr       */
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
use crate::puzzle::PuzzleRes;
use crate::options::Options;

fn update_open_list(puzzle: & Vec<u16>, open_list: &mut BinaryHeap<PuzzleRes>, final_state: & puzzle::FinalPuzzle, all_list: &mut HashSet<Vec<u16>>, actual_dst: i32, opts: &Options) {
	let zero_pos = puzzle.iter().position(|r| *r == 0).unwrap() as u16 as usize;
	static MOVE_FUNCTIONS: &[ fn(&Vec<u16>, usize, usize) -> Result<Vec<u16>, io::Error>; 4] =
		&[puzzle::move_up, puzzle::move_down, puzzle::move_left, puzzle::move_right];
	let mut dst;

	for function in MOVE_FUNCTIONS {
		match function(&puzzle, zero_pos, final_state.size as usize) {
			Ok(taquin) => {
				if !all_list.contains(&taquin) {
					dst = heuristics::distance_estimator(&taquin, final_state, opts);
					open_list.push(PuzzleRes{taq: taquin.clone(),estimate_dst: dst, actual_dst: actual_dst + 1,predecessor: puzzle.clone()} );
					all_list.insert(taquin);
				}
			}
			_          => (),
		}
	}
}

pub fn solve(p: &Vec<u16>, final_state: &puzzle::FinalPuzzle, opts: & Options) {
	let mut close_list: HashMap<Vec<u16>, (i32, i32, Vec<u16>)> = HashMap::new();
	let mut open_list: BinaryHeap<PuzzleRes>  = BinaryHeap::new();
	let mut all_list: HashSet<Vec<u16>> = HashSet::new();
	let mut puzzle = p;
	let ref mut o_list = open_list;
	let ref mut a_list = all_list;
	let mut next = PuzzleRes {
		taq: Vec::new(),
		estimate_dst: heuristics::distance_estimator(puzzle, final_state, opts) as i32,
		actual_dst: 0,
		predecessor:Vec::new()
	};

	let start = Instant::now();
	a_list.insert(puzzle.clone());
	// while puzzle.estimate_dst != 0 {
	while *puzzle != final_state.puzzle {
		close_list.insert(puzzle.clone(), (next.estimate_dst, next.actual_dst, next.predecessor));
		update_open_list(puzzle, o_list, final_state, a_list, next.actual_dst, opts);
		next = o_list.pop().unwrap();
		puzzle = &next.taq;
		if opts.greedy {
			next.actual_dst = 0;
		}
	}
	println! ("Success :)");
	let time = start.elapsed();
	print::print_all(puzzle, &close_list, &next.predecessor, final_state, opts);
	println!("Time  : {:?}", time);
}