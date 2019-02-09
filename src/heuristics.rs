/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   heuristics.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/05 10:34:16 by no                #+#    #+#             */
/*   Updated: 2019/02/09 20:55:50 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// use crate::puzzle::Puzzle;
use crate::options::HeuristicType;
use crate::options::Options;
use crate::puzzle;


use std::process;

pub fn estimate_one_manhattan(taquin: & Vec<u16>, final_state: &puzzle::FinalPuzzle, value: u16, size: i32) -> i32 {
	let pos_current = taquin.iter().position(|r| *r == value).unwrap() as i32;
	// let pos_final = final_state.get_pos_of_value(value) as i32;

	((((pos_current % size) - ((final_state.position[value as usize] as i32) % size)) as i32).abs()
		+ (((pos_current / size) - ((final_state.position[value as usize] as i32) / size)) as i32).abs()) as i32
}


pub fn distance_estimator_manhattan(taquin: & Vec<u16>, final_state: &puzzle::FinalPuzzle) -> i32 {
	let mut cmpt: i32 = 0;
	let size = final_state.size;
	let sq: usize = (size * size) as usize;

	for i in 1..sq {
		cmpt += estimate_one_manhattan(taquin, final_state, i as u16, size);
	}
	cmpt
}

pub fn distance_estimator_hamming(taquin: & Vec<u16>, final_state: &puzzle::FinalPuzzle) -> i32 {
	let mut cmpt: i32 = 0;
	let sq: usize = (final_state.size * final_state.size) as usize;

	for i in 1..sq - 1 {
		if taquin[i] != final_state.puzzle[i] {
			cmpt += 1;
		}
	}
	cmpt
}

pub fn distance_estimator_linear(taquin: & Vec<u16>, final_state: &puzzle::FinalPuzzle, opts: & Options) -> i32 {
	/////////
	let mut cmpt: i32 = 0;
	let size = final_state.size as usize;
	let sq: usize = (size * size) - 1;

	println!("distance_estimator_linear: \ncurrent:");
	puzzle::print_puzzle(&taquin, size, final_state, opts);
	println!("final:");
	puzzle::print_puzzle(&final_state.puzzle, size, final_state, opts);

	let mut current_pos_x: usize;
	let mut current_pos_y: usize;
	let mut final_pos_x: usize;
	let mut final_pos_y: usize;
	let mut current_value: usize;

	for i in 0..sq {
		current_value = taquin[i] as usize;
		if current_value == 0 { continue; }
		current_pos_x = i % size;
		current_pos_y = i / size;
		final_pos_x = final_state.position[current_value] as usize % size;
		final_pos_y = final_state.position[current_value] as usize / size;
		if current_pos_y == final_pos_y { //same line
			for i2 in current_pos_x..size {
				if i2 == i % size { continue; }
				if final_state.position[i2] as usize / size == current_pos_y && final_pos_x > final_state.position[taquin[i2] as usize] as usize % size {
				println!("Conflict (horizontal): {} <-> {}", taquin[i], taquin[i2]);
					cmpt += 2;
				}
				else {
					println!("Not Conflict (horizontal): {} <-> {}", taquin[i], taquin[i2]);
				}
			}
		}
		if current_pos_x == final_pos_x { //same row
			// for i2 in (current_pos_y..sq).filter(|x| x % size == current_pos_y) {
			let mut i2 = current_pos_y;
			while i2 < sq {
				if i2 == i / size { i2 += size; continue; }
				if final_state.position[i2]  as usize % size == current_pos_x && final_pos_y > final_state.position[taquin[i2] as usize] as usize / size {
				println!("Conflict (vertical): {} <-> {}", taquin[i], taquin[i2]);
					cmpt += 2;
				}
				else {
					println!("NOT Conflict (vertical): {} <-> {}", taquin[i], taquin[i2]);
				}
				i2 += size;
			}
		}
		cmpt += estimate_one_manhattan(taquin, final_state, i as u16, size as i32);
	}
	// cmpt
	// FUCK


	println!("distance_estimator_linear: FINAL :{}", cmpt);
	process::exit(0);
	cmpt
}

pub fn distance_estimator_combine(taquin: & Vec<u16>, final_state: &puzzle::FinalPuzzle) -> i32 {
	let mut cmpt: i32 = 0;
	let size = final_state.size;
	let sq: usize = (size * size) as usize - 1;

	for i in 1..sq {
		cmpt += estimate_one_manhattan(taquin, final_state, i as u16, size);
		if taquin[i] != final_state.puzzle[i] {
			cmpt += 1;
		}
	}
	cmpt
}

pub fn distance_estimator(taquin: & Vec<u16>, final_state: &puzzle::FinalPuzzle, opts: & Options) -> i32 {
	match opts.heuristic {
		HeuristicType::Manhattan => distance_estimator_manhattan(taquin, final_state),
		HeuristicType::Hamming   => distance_estimator_hamming(taquin, final_state),
		HeuristicType::Linear    => distance_estimator_linear(taquin, final_state, opts),
		HeuristicType::Combine   => distance_estimator_combine(taquin, final_state),
	}
}
