/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   solver.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/05 10:34:10 by no                #+#    #+#             */
/*   Updated: 2019/02/13 20:57:38 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::time::Instant;
use crate::puzzle;
use crate::print;
use crate::options::Options;
use crate::puzzle::RefPuzzle;

fn update_open_list(r_puzzle: RefPuzzle, open_list: &mut BinaryHeap<RefPuzzle>, final_state: & puzzle::FinalPuzzle, all_list: &mut HashSet<RefPuzzle>, opts: &Options, actual_dst: usize) {
	let zero_pos = r_puzzle.ref_puzzle.borrow().taq.iter().position(|r| *r == 0).unwrap() as u16 as usize;

static MOVE_FUNCTIONS: &[ fn(usize, & puzzle::FinalPuzzle, &RefPuzzle, &Options, i32) -> Result<RefPuzzle, io::Error>; 4] =
		&[puzzle::move_up, puzzle::move_down, puzzle::move_left, puzzle::move_right];

	for function in MOVE_FUNCTIONS {
		match function(zero_pos, final_state, &r_puzzle, opts, actual_dst as i32) {
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

pub fn solve(first: Vec<u16>, final_state: &puzzle::FinalPuzzle, opts: & Options) {

	let mut close_list: HashSet<RefPuzzle> = HashSet::new();
	let mut open_list: BinaryHeap<RefPuzzle> = BinaryHeap::new();
	let mut all_list: HashSet<RefPuzzle> = HashSet::new();
	let ref mut o_list = open_list;
	let ref mut a_list = all_list;
	let mut puzzle = RefPuzzle::first(first, final_state, opts);
	let mut actual_dst = 0;
	
	let not_greedy = if opts.greedy { 0 } else { 1 };

	let start = Instant::now();
	a_list.insert(puzzle.clone());
	close_list.insert(puzzle.clone());

	while puzzle.ref_puzzle.borrow().taq != final_state.puzzle {
		update_open_list(puzzle, o_list, final_state, a_list, opts, actual_dst + not_greedy);
		puzzle = o_list.pop().unwrap();
		actual_dst = puzzle.ref_puzzle.borrow().actual_dst as usize;
		close_list.insert(puzzle.clone());
	}
	let time = start.elapsed();
	println! ("Success :)");
	print::print(&close_list, open_list.len() + close_list.len(), &puzzle, final_state, opts);
	println!("# Time  : {:?}\n###", time);
}