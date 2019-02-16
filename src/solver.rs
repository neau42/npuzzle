/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   solver.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nboulaye <nboulaye@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/05 10:34:10 by no                #+#    #+#             */
/*   Updated: 2019/02/16 18:04:19 by nboulaye         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::options::Options;
use crate::print;
use crate::puzzle;
use crate::puzzle::RefPuzzle;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::io;
use std::time::Instant;

fn update_open_list(
    r_puzzle: RefPuzzle,
    open_list: &mut BinaryHeap<RefPuzzle>,
    final_state: &puzzle::FinalPuzzle,
    all_list: &mut HashSet<RefPuzzle>,
    opts: &Options,
    actual_dst: usize,
) {
    let zero_pos = r_puzzle
        .ref_puzzle
        .borrow()
        .taq
        .iter()
        .position(|r| *r == 0)
        .unwrap() as u16 as usize;

    static MOVE_FUNCTIONS: &[fn(usize, &puzzle::FinalPuzzle, &RefPuzzle, &Options, i32) -> Result<RefPuzzle, io::Error>;
         4] = &[
        puzzle::move_up,
        puzzle::move_down,
        puzzle::move_left,
        puzzle::move_right,
    ];

    for function in MOVE_FUNCTIONS {
        if let Ok(new_puzzle) = function(zero_pos, final_state, &r_puzzle, opts, actual_dst as i32)
        {
            if !all_list.contains(&new_puzzle) {
                open_list.push(new_puzzle.clone());
                all_list.insert(new_puzzle);
            }
        }
    }
}

pub fn solve(first: Vec<u16>, final_state: &puzzle::FinalPuzzle, opts: &Options) {
    // let mut close_list: HashSet<RefPuzzle> = HashSet::new();
    let mut open_list: BinaryHeap<RefPuzzle> = BinaryHeap::new();
    let mut all_list: HashSet<RefPuzzle> = HashSet::new();
    let mut puzzle = RefPuzzle::first(first, final_state, opts);
    let mut actual_dst = 0;

    let not_greedy = if opts.greedy { 0 } else { 1 };

    let start = Instant::now();
    all_list.insert(puzzle.clone());
    // close_list.insert(puzzle.clone());

    while puzzle.ref_puzzle.borrow().taq != final_state.puzzle {
        update_open_list(
            puzzle,
            &mut open_list,
            final_state,
            &mut all_list,
            opts,
            actual_dst + not_greedy,
        );
        puzzle = open_list.pop().unwrap();
        actual_dst = puzzle.ref_puzzle.borrow().actual_dst as usize;

	puzzle.ref_puzzle.borrow_mut().in_close = true;
        // close_list.insert(puzzle.clone());
    }
    let time = start.elapsed();
    println!("Success :)");
    print::print(
        &all_list,
        all_list.len(),
        &puzzle,
        final_state,
        opts,
    );
    println!("# Time  : {:?}\n###", time);
}
