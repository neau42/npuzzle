/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   solver.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/05 10:34:10 by no                #+#    #+#             */
/*   Updated: 2019/02/12 21:28:40 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;
// use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::time::Instant;
use crate::heuristics;
use crate::puzzle;
use crate::print;
use crate::puzzle::PuzzleRes;
use crate::options::Options;

use crate::puzzle::RefPuzzle;
use std::rc::Rc;
use std::cell::RefCell;

// use std::process;
fn update_open_list(r_puzzle: & RefPuzzle, open_list: &mut BinaryHeap<RefPuzzle>, final_state: & puzzle::FinalPuzzle, all_list: &mut HashSet<RefPuzzle>, opts: &Options, mut actual_dst: usize) {
	let zero_pos = r_puzzle.ref_puzzle.borrow().taq.iter().position(|r| *r == 0).unwrap() as u16 as usize;

static MOVE_FUNCTIONS: &[ fn(&Vec<u16>, usize, & puzzle::FinalPuzzle, &RefPuzzle, &Options, i32) -> Result<RefPuzzle, io::Error>; 4] =
		&[puzzle::move_up, puzzle::move_down, puzzle::move_left, puzzle::move_right];

	for function in MOVE_FUNCTIONS {
		match function(&r_puzzle.ref_puzzle.borrow().taq, zero_pos, final_state, r_puzzle, opts, actual_dst as i32) {
			Ok(new_puzzle) => {
				if !all_list.contains(&new_puzzle) {
					open_list.push(new_puzzle.clone());
					all_list.insert(new_puzzle);
				}
			}
			_          => (),
		}
	}
}

pub fn solve(p: &Vec<u16>, final_state: &puzzle::FinalPuzzle, opts: & Options) {

	let mut close_list_new: HashSet<RefPuzzle> = HashSet::new();
	let mut open_list_new: BinaryHeap<RefPuzzle> = BinaryHeap::new();
	let mut all_list_new: HashSet<RefPuzzle> = HashSet::new();
	let ref mut o_list = open_list_new;
	let ref mut a_list = all_list_new;

	let first_puzzle: RefPuzzle = RefPuzzle {
			ref_puzzle :Rc::new(RefCell::new(PuzzleRes {
				taq: p.clone(),
				estimate_dst: heuristics::distance_estimator(p, final_state, opts) as i32,
				actual_dst: 0,
				predecessor: None,
			} ))
		};
	let mut puzzle = first_puzzle;
	let mut actual_dst = 0;

	let start = Instant::now();
	a_list.insert(puzzle.clone());
	println!("first:");
	puzzle::print_puzzle(& a_list.get(&puzzle).unwrap().ref_puzzle.borrow().taq, final_state, opts);
	let test_greedy = if opts.greedy { 0 } else { 1 };
	
	while puzzle.ref_puzzle.borrow().taq != final_state.puzzle {
		close_list_new.insert(puzzle.clone());
		update_open_list(&puzzle, o_list, final_state, a_list, opts, actual_dst + test_greedy);
		puzzle = o_list.pop().unwrap();
	}
	close_list_new.insert(puzzle.clone());
	println! ("Success :)");
	let time = start.elapsed();
	print::print(&close_list_new, &puzzle, final_state, opts);
	// print::print_all(puzzle, &close_list, &next.predecessor, final_state, opts);
	println!("Time  : {:?}", time);
}