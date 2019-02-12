/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   heuristics.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/05 10:34:16 by no                #+#    #+#             */
/*   Updated: 2019/02/11 19:09:52 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::options::HeuristicType;
use crate::options::Options;
use crate::puzzle;

// use std::process;

pub fn estimate_one_manhattan(taquin: & Vec<u16>, final_state: &puzzle::FinalPuzzle, value: u16, size: i32) -> i32 {
	let pos_current = taquin.iter().position(|r| *r == value).unwrap() as i32;

	((((pos_current % size) - ((final_state.position[value as usize] as i32) % size))).abs()
		+ (((pos_current / size) - ((final_state.position[value as usize] as i32) / size))).abs()) as i32
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

pub fn distance_estimator_linear(taquin: & Vec<u16>, final_state: &puzzle::FinalPuzzle) -> i32 {
	let size = final_state.size as usize;
	let sq: usize = (size * size) - 1;

	let mut cmpt: i32 = 0;
	let mut current_pos_x: usize;
	let mut current_pos_y: usize;
	let mut final_pos_x: usize;
	let mut final_pos_y: usize;
	let mut current_value: usize;

	// println!("current: ");
	// puzzle::print_puzzle(&taquin, size as usize, final_state, opts);
	// println!("final: ");
	// puzzle::print_puzzle(&final_state.puzzle, size as usize, final_state, opts);

	for i in 0..sq {
		current_value = taquin[i] as usize;
		if current_value == 0 || final_state.position[current_value] as usize == current_value { continue; }
		current_pos_x = i % size;
		current_pos_y = i / size;
		final_pos_x = final_state.position[current_value] as usize % size;
		final_pos_y = final_state.position[current_value] as usize / size;
		if current_pos_y == final_pos_y { //same line
			for i2 in (i + 1)..(final_pos_y * size + size) {
				if taquin[i2] != 0 && final_state.position[taquin[i2] as usize] as usize / size == current_pos_y 
				&& final_state.position[taquin[i2] as usize] as usize % size < final_pos_x {
					// println!("conflict (vertical) {} <--> {}", taquin[i], taquin[i2]);
					cmpt += 2;
				}
			}
		}
		if current_pos_x == final_pos_x { //same raw
			let mut i2 = i + size;
			while i2 < sq {
				if taquin[i2] != 0 && final_state.position[taquin[i2] as usize] as usize % size == current_pos_x 
				&& final_state.position[taquin[i2] as usize] as usize / size < final_pos_y {
					// println!("conflict (horizontal) {} <--> {}", taquin[i], taquin[i2]);
					cmpt += 2;
				}
				i2 += size;
			}
		}
	}
	// process::exit(0);
	// (cmpt + distance_estimator_manhattan(taquin, final_state))
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
		HeuristicType::Linear    => distance_estimator_linear(taquin, final_state) + distance_estimator_manhattan(taquin, final_state),
		HeuristicType::Combine   => distance_estimator_combine(taquin, final_state),
	}
}
