/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   puzzle.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/05 10:34:18 by no                #+#    #+#             */
/*   Updated: 2019/02/05 11:11:45 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;
use std::io::ErrorKind as IoErr;
use std::cmp::Ordering;
use crate::heuristics;

#[derive(Debug, Eq, Hash)]
pub struct PuzzleRes {
    pub taq: Vec<u8>,
	pub estimate_dst: i32,    //  H
	pub actual_dst: i32, //  G
	pub predecessor: Vec<u8>,
}

impl PartialEq for PuzzleRes {
		fn eq(&self, other: &PuzzleRes) -> bool {
			self.taq == other.taq
		}
	}

impl PartialOrd for PuzzleRes {
    fn partial_cmp(&self, other: &PuzzleRes) -> Option<Ordering> {
		if self.estimate_dst + self.actual_dst == other.estimate_dst + other.actual_dst {
			return (other.actual_dst).partial_cmp(&(self.actual_dst));
		}
		(other.estimate_dst + other.actual_dst).partial_cmp(&(self.estimate_dst + self.actual_dst))
	}
}

impl Ord for PuzzleRes {
     fn cmp(&self, other: &PuzzleRes) -> Ordering {
		if self.estimate_dst + self.actual_dst == other.estimate_dst + other.actual_dst {
			return (other.actual_dst).cmp(&(self.actual_dst));
		}
		(other.estimate_dst + other.actual_dst).cmp(&(self.estimate_dst + self.actual_dst))
	}
}

#[derive(Debug, Eq, Hash)]
pub struct Puzzle {
    pub size: i32,
    pub taq: Vec<u8>,
	pub actual_dst: i32, //  G
	pub estimate_dst: i32,    //  H
}

impl PartialEq for Puzzle {
		fn eq(&self, other: &Puzzle) -> bool {
			self.taq == other.taq
		}
	}

impl PartialOrd for Puzzle {
    fn partial_cmp(&self, other: &Puzzle) -> Option<Ordering> {
		(self.estimate_dst + self.actual_dst).partial_cmp(&(other.estimate_dst + other.actual_dst))
    }
}

impl Ord for Puzzle {
    fn cmp(&self, other: &Puzzle) -> Ordering {
		(self.estimate_dst + self.actual_dst).cmp(&(other.estimate_dst + other.actual_dst))
    }
}

impl Puzzle {
	pub fn new(size: i32, taq: Vec<u8>, actual_dst: i32, estimate_dst: i32) -> Puzzle {
		Puzzle { size, taq, actual_dst, estimate_dst}
	}

	pub fn copy(&self) -> Puzzle {
		Puzzle { size: self.size as i32, taq : self.taq.clone(), actual_dst: self.actual_dst, estimate_dst: self.estimate_dst }
	}

	pub fn gen_final_state(size: usize) -> Puzzle {
		let sq: usize = size * size;
		let mut v = vec![0; sq];
		let mut horizontal: bool = true;
		let mut index: i32 = -1;
		let mut inverse: bool = false;
		let mut cmpt: usize = size;
		let mut cmpt_ref: usize = size;

		for value in 1..sq {
			if cmpt == 0 {
				inverse = size % 2 == (cmpt_ref - 1) % 2;
				horizontal = !horizontal;
				if !horizontal {
					cmpt_ref -= 1;
				}
				cmpt = cmpt_ref;
			}
			index = match (horizontal, inverse) {
				(true, false)  => index + 1,
				(true, true)   => index - 1,
				(false, false) => index + size as i32,
				(false, true)  => index - size as i32,
			};
			v[index as usize] = value as u8;
			cmpt -= 1;
		}
		Puzzle { size: size as i32, taq : v, actual_dst: -1, estimate_dst: 0 }
	}

	pub fn get_pos_of_value(&self, value: u8) -> u8 {
		self.taq.iter().position(|r| *r == value).unwrap() as u8
	}

	pub fn is_valid(&self) -> bool {
		let sq: u8 = (self.size * self.size) as u8;
		let mut v: Vec<u8> = Vec::new();

		if self.taq.len() != sq as usize {
			return false;
		}
		for e in &self.taq {
			if *e >= sq || v.iter().any(|x| *x == *e) {
				return false;
			}
			v.push(*e);
		}
		true
	}

	pub fn is_soluble(&self) -> bool {
		let mut cmpt: i32 = 0;
		let mut vect_copy: Vec<u8> = self.taq.clone();
		let sq: usize = (self.size * self.size) as usize - 1;
		let final_state = Puzzle::gen_final_state(self.size as usize);

		for idx in 0..sq {
			if vect_copy[idx as usize] != final_state.taq[idx as usize] {
				let pos = vect_copy.iter().enumerate().find(|r| *r.1 == final_state.taq[idx as usize]).unwrap().0;
				vect_copy.swap(idx as usize, (pos) as usize);
				cmpt += 1;
			}
		}
		(heuristics::estimate_one_manhattan(&self.taq, &final_state, 0, final_state.size) % 2 == cmpt % 2)		
	}
}

pub fn move_left(taquin: & Vec<u8>, zero_pos: usize, size: usize) -> Result<Vec<u8>, io::Error> {
	if !(zero_pos % size == size - 1) {
		let mut new = taquin.clone();
		new.swap(zero_pos, zero_pos + 1);
		return Ok(new);
	}
	Err(std::io::Error::new(IoErr::Other, "unable to move left"))
}

pub fn move_right(taquin: & Vec<u8>, zero_pos: usize, size: usize) -> Result<Vec<u8>, io::Error> {
	if !(zero_pos % size == 0) {
		let mut new = taquin.clone();
		new.swap(zero_pos, zero_pos - 1);
		return Ok(new);
	}
	Err(std::io::Error::new(IoErr::Other, "unable to move right"))
}
pub fn move_up(taquin: & Vec<u8>, zero_pos: usize, size: usize) -> Result<Vec<u8>, io::Error> {
	if !(zero_pos >= size * size - size) {
		let mut new = taquin.clone();
		new.swap(zero_pos, zero_pos + size);
		return Ok(new);
	}
	Err(std::io::Error::new(IoErr::Other, "unable to move up"))
}

pub fn move_down(taquin: & Vec<u8>, zero_pos: usize, size: usize) -> Result<Vec<u8>, io::Error> {
	if !(zero_pos < size) {
		let mut new = taquin.clone();
		new.swap(zero_pos, zero_pos - size);
		return Ok(new);
	}
	Err(std::io::Error::new(IoErr::Other, "unable to move down"))
}

pub fn print_puzzle(taquin: & Vec<u8>, size: usize) {
	let sq: usize = (size * size) as usize;

	for i in 0..sq {
		print!("{number:>width$} ", number=taquin[i], width=2);
		if i % (size as usize) == size as usize - 1 {
			print!("\n");
		}
	}
	print!("\n");
}
