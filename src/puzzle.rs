/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   puzzle.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nboulaye <nboulaye@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/05 10:34:18 by no                #+#    #+#             */
/*   Updated: 2019/02/16 18:03:29 by nboulaye         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::heuristics;
use crate::options::Options;
use heuristics::manhattan;

use std::cmp::Ordering;
use std::io;
use std::io::ErrorKind as IoErr;

use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Eq)]
pub struct RefPuzzle {
    pub ref_puzzle: Rc<RefCell<PuzzleRes>>,
}

impl Clone for RefPuzzle {
    fn clone(&self) -> RefPuzzle {
        RefPuzzle {
            ref_puzzle: Rc::clone(&self.ref_puzzle),
        }
    }
}

impl PartialOrd for RefPuzzle {
    fn partial_cmp(&self, other: &RefPuzzle) -> Option<Ordering> {
        (other.ref_puzzle.borrow().total_dst).partial_cmp(&(self.ref_puzzle.borrow().total_dst))
    }
}

impl PartialEq for RefPuzzle {
    fn eq(&self, other: &RefPuzzle) -> bool {
        self.ref_puzzle.borrow().taq == other.ref_puzzle.borrow().taq
    }
}

impl Ord for RefPuzzle {
    fn cmp(&self, other: &RefPuzzle) -> Ordering {
        (other.ref_puzzle.borrow().total_dst).cmp(&(self.ref_puzzle.borrow().total_dst))
    }
}

impl Hash for RefPuzzle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ref_puzzle.borrow().hash(state);
    }
}
impl RefPuzzle {
    pub fn new(
        taquin: Vec<u16>,
        final_state: &FinalPuzzle,
        predecessor: RefPuzzle,
        opts: &Options,
        actual_dst: i32,
    ) -> RefPuzzle {
        let estimate_dst = heuristics::distance_estimator(&taquin, final_state, opts);
        RefPuzzle {
            ref_puzzle: Rc::new(RefCell::new(PuzzleRes {
                estimate_dst,
                actual_dst,
                total_dst: actual_dst + estimate_dst,
                predecessor: Some(predecessor),
                taq: taquin,
				in_close: false
            })),
        }
    }

    pub fn first(taquin: Vec<u16>, final_state: &FinalPuzzle, opts: &Options) -> RefPuzzle {
        let estimate_dst = heuristics::distance_estimator(&taquin, final_state, opts);
        RefPuzzle {
            ref_puzzle: Rc::new(RefCell::new(PuzzleRes {
                estimate_dst,
                taq: taquin,
                actual_dst: 0,
                total_dst: estimate_dst,
                predecessor: None,
				in_close: false
            })),
        }
    }
}
#[derive(Debug, Eq)]
pub struct PuzzleRes {
    pub taq: Vec<u16>,
    pub estimate_dst: i32,
    pub actual_dst: i32,
    pub total_dst: i32,
    pub predecessor: Option<RefPuzzle>,
	pub in_close: bool,
}

impl Hash for PuzzleRes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.taq.hash(state);
    }
}

impl PartialEq for PuzzleRes {
    fn eq(&self, other: &PuzzleRes) -> bool {
        self.taq == other.taq
    }
}

impl PartialOrd for PuzzleRes {
    fn partial_cmp(&self, other: &PuzzleRes) -> Option<Ordering> {
        (other.total_dst).partial_cmp(&(self.total_dst))
    }
}

impl Ord for PuzzleRes {
    fn cmp(&self, other: &PuzzleRes) -> Ordering {
        (other.total_dst).cmp(&(self.total_dst))
    }
}

#[derive(Debug, Hash)]
pub struct Puzzle {
    pub size: i32,
    pub taq: Vec<u16>,
}

impl Puzzle {
    pub fn new(size: i32, taq: Vec<u16>) -> Puzzle {
        Puzzle { size, taq }
    }

    pub fn copy(&self) -> Puzzle {
        Puzzle {
            size: self.size as i32,
            taq: self.taq.clone(),
        }
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
                (true, false) => index + 1,
                (true, true) => index - 1,
                (false, false) => index + size as i32,
                (false, true) => index - size as i32,
            };
            v[index as usize] = value as u16;
            cmpt -= 1;
        }
        Puzzle {
            size: size as i32,
            taq: v,
        }
    }

    pub fn get_pos_of_value(&self, value: u16) -> u16 {
        self.taq.iter().position(|r| *r == value).unwrap() as u16
    }

    pub fn is_valid(&self) -> bool {
        let sq: u16 = (self.size * self.size) as u16;
        let mut v: Vec<u16> = Vec::new();

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

    pub fn is_soluble(&self, final_state: &FinalPuzzle) -> bool {
        let mut cmpt: i32 = 0;
        let mut vect_copy: Vec<u16> = self.taq.clone();
        let sq: usize = (self.size * self.size) as usize - 1;

        for idx in 0..sq {
            if vect_copy[idx as usize] != final_state.puzzle[idx as usize] {
                let pos = vect_copy
                    .iter()
                    .enumerate()
                    .find(|r| *r.1 == final_state.puzzle[idx as usize])
                    .unwrap()
                    .0;
                vect_copy.swap(idx as usize, (pos) as usize);
                cmpt += 1;
            }
        }
        (manhattan::estimate_one(
            (final_state.position[0] as i32) % final_state.size,
            (final_state.position[0] as i32) / final_state.size,
            &self.taq,
            0,
            final_state.size,
        ) % 2
            == cmpt % 2)
    }
}

#[derive(Debug)]
pub struct FinalPuzzle {
    pub size: i32,
    pub puzzle: Vec<u16>,
    pub position: Vec<u16>,
}

pub fn init_final_stat(size: usize) -> FinalPuzzle {
    let puzzle = Puzzle::gen_final_state(size);
    let sq: u16 = (size * size) as u16;
    let mut position: Vec<u16> = vec![0; sq as usize];

    for i in 0..sq {
        position[i as usize] = puzzle.get_pos_of_value(i);
    }
    FinalPuzzle {
        size: size as i32,
        puzzle: puzzle.taq,
        position,
    }
}

pub fn move_up(
    zero_pos: usize,
    final_state: &FinalPuzzle,
    predecessor: &RefPuzzle,
    opts: &Options,
    actual_dst: i32,
) -> Result<RefPuzzle, io::Error> {
    if zero_pos < final_state.size as usize * final_state.size as usize - final_state.size as usize
    {
        let mut new_taquin = predecessor.ref_puzzle.borrow().taq.clone();
        new_taquin.swap(zero_pos, zero_pos + final_state.size as usize);
        return Ok(RefPuzzle::new(
            new_taquin,
            final_state,
            predecessor.clone(),
            opts,
            actual_dst,
        ));
    }
    Err(std::io::Error::new(IoErr::Other, "unable to move up"))
}

pub fn move_down(
    zero_pos: usize,
    final_state: &FinalPuzzle,
    predecessor: &RefPuzzle,
    opts: &Options,
    actual_dst: i32,
) -> Result<RefPuzzle, io::Error> {
    if zero_pos >= final_state.size as usize {
        let mut new_taquin = predecessor.ref_puzzle.borrow().taq.clone();
        new_taquin.swap(zero_pos, zero_pos - final_state.size as usize);
        return Ok(RefPuzzle::new(
            new_taquin,
            final_state,
            predecessor.clone(),
            opts,
            actual_dst,
        ));
    }
    Err(std::io::Error::new(IoErr::Other, "unable to move down"))
}

pub fn move_right(
    zero_pos: usize,
    final_state: &FinalPuzzle,
    predecessor: &RefPuzzle,
    opts: &Options,
    actual_dst: i32,
) -> Result<RefPuzzle, io::Error> {
    if zero_pos % final_state.size as usize != 0 {
        let mut new_taquin = predecessor.ref_puzzle.borrow().taq.clone();
        new_taquin.swap(zero_pos, zero_pos - 1);
        return Ok(RefPuzzle::new(
            new_taquin,
            final_state,
            predecessor.clone(),
            opts,
            actual_dst,
        ));
    }
    Err(std::io::Error::new(IoErr::Other, "unable to move right"))
}

pub fn move_left(
    zero_pos: usize,
    final_state: &FinalPuzzle,
    predecessor: &RefPuzzle,
    opts: &Options,
    actual_dst: i32,
) -> Result<RefPuzzle, io::Error> {
    if zero_pos % final_state.size as usize != final_state.size as usize - 1 {
        let mut new_taquin = predecessor.ref_puzzle.borrow().taq.clone();
        new_taquin.swap(zero_pos, zero_pos + 1);
        return Ok(RefPuzzle::new(
            new_taquin,
            final_state,
            predecessor.clone(),
            opts,
            actual_dst,
        ));
    }
    Err(std::io::Error::new(IoErr::Other, "unable to move left"))
}
