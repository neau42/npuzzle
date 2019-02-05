/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   solver.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/05 10:34:10 by no                #+#    #+#             */
/*   Updated: 2019/02/05 10:34:27 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// use std::thread;
// use std::time;
use std::io;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use crate::heuristics;

use crate::puzzle;
use crate::puzzle::Puzzle;
use crate::puzzle::PuzzleRes;

fn update_open_list(puzzle: & Puzzle, open_list: &mut  BinaryHeap<PuzzleRes> , final_state: & Puzzle, all_list: &mut HashSet<Vec<u8>>) {
	let zero_pos = puzzle.get_pos_of_value(0) as usize;
	static MOVE_FUNCTIONS: &[ fn(&Vec<u8>, usize, usize) -> Result<Vec<u8>, io::Error>; 4] =
		&[puzzle::move_up, puzzle::move_down, puzzle::move_left, puzzle::move_right];

	for function in MOVE_FUNCTIONS {
		match function(&puzzle.taq, zero_pos, puzzle.size as usize) {
			Ok(taquin) => {
				let dst = heuristics::distance_estimator(&taquin, final_state);
				if !all_list.contains(&taquin) {
					open_list.push(PuzzleRes{taq: taquin.clone(),estimate_dst: dst, actual_dst: puzzle.actual_dst + 1,predecessor: puzzle.taq.clone()} );
					all_list.insert(taquin.clone());
				}
			}
			_ =>  (),
		}
	}
}

fn print_all(puzzle: &mut Puzzle, close_list: &HashMap<Vec<u8>, (i32, i32, Vec<u8>)>, size: i32, mut predecessor: Vec<u8>) -> u32 {
	let ref mut end: Vec<Vec<u8>> = Vec::new();
	let mut i: u32 = 0;

	loop {
		end.push(puzzle.taq.clone());
		puzzle.taq = predecessor.clone();
		predecessor = match close_list.get(&predecessor) {
			Some(x) => x.2.clone(),
			_ => break,
		};
		i += 1;
	}
	loop {
		let p = match end.pop() {
			Some(x) => x,
			_ => break, 
		};
		println!("N-puzzle: ");
		puzzle::print_puzzle(&p, size as usize);
			// thread::sleep(time::Duration::from_millis(250));
	}
	i
}

pub fn solve(puzzle: &mut Puzzle) {
	let final_state = Puzzle::gen_final_state(puzzle.size as usize);
	let mut close_list: HashMap<Vec<u8>, (i32, i32, Vec<u8>)> = HashMap::new();
	let mut open_list: BinaryHeap<PuzzleRes>  = BinaryHeap::new();
	let mut all_list: HashSet<Vec<u8>> = HashSet::new();

	let ref mut c_list = close_list;
	let ref mut o_list = open_list;
	let ref mut a_list = all_list;

	let mut next = PuzzleRes{taq: Vec::new(), estimate_dst: 0 , actual_dst:0 , predecessor:Vec::new() };

	a_list.insert(puzzle.taq.clone());

	while puzzle.estimate_dst != 0 {
		c_list.insert(puzzle.taq.clone(), (puzzle.estimate_dst, puzzle.actual_dst, next.predecessor.clone()));
		update_open_list(puzzle, o_list, &final_state, a_list);
		next = o_list.pop().unwrap();
		puzzle.taq = next.taq;
		puzzle.actual_dst = next.actual_dst;
		puzzle.estimate_dst = next.estimate_dst;
	}
	println! ("Success :)");
	let len = print_all(puzzle, c_list, final_state.size, next.predecessor.clone());
	println! ("len: {} ", len);
	println! ("Open_list.len() : {} ", o_list.len());
	println! ("Close_list.len() : {} ", c_list.len() + 1);
	println! ("All_list.len() : {}", a_list.len());
	// println! ("nb states: {} ", nb_state); // == c_list.len()
	// println! ("max rep: {} ", max_state); // == o_list.len()
}