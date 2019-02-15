/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   print.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/13 21:05:46 by no                #+#    #+#             */
/*   Updated: 2019/02/15 18:59:22 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::puzzle;
use crate::puzzle::RefPuzzle;
use std::collections::HashSet;
use std::thread;
use std::time;
extern crate termion;
use crate::options::Options;

pub fn print_puzzle(taquin: &[u16], final_state: &puzzle::FinalPuzzle, opts: &Options) {
    let size = final_state.size;
    let sq: usize = (size * size) as usize;

    // for i in 0..sq {
		for (i, _) in taquin.iter().enumerate().take(sq) {
        if opts.color {
            if taquin[i] == 0 {
            } else if final_state.position[taquin[i] as usize] == i as u16 {
                print!("{}[92m", 27 as char);
            } else {
                print!("{}[91m", 27 as char);
            }
            print!("{number:>width$} ", number = taquin[i], width = 2);
            print!("{}[0m", 27 as char);
        } else {
            print!("{number:>width$} ", number = taquin[i], width = 2);
        }
        if i % (size as usize) == size as usize - 1 {
            println!();
        }
    }
    println!();
}

pub fn print(
    close_list: &HashSet<RefPuzzle>,
    nb_states: usize,
    puzzle: &RefPuzzle,
    final_state: &puzzle::FinalPuzzle,
    opts: &Options,
) {
    let mut list_final: Vec<Vec<u16>> = Vec::new();
    let mut ref_predecessor;
    let mut predecessor;
    let mut taquin;
    let mut len: u16 = 0;

    list_final.push(final_state.puzzle.clone());
    let mut puzzle = puzzle.ref_puzzle.borrow();
    loop {
        ref_predecessor = match &puzzle.predecessor {
            Some(ref_next) => ref_next,
            _ => break,
        };
        predecessor = close_list.get(&ref_predecessor).unwrap();
        puzzle = predecessor.ref_puzzle.borrow();
        taquin = &puzzle.taq;
        list_final.push(taquin.clone());
        len += 1;
    }
    let mut elem: Vec<u16>;
    loop {
        elem = match list_final.pop() {
            Some(x) => x,
            _ => break,
        };
        if opts.sleep {
            println!("{}[2J{}", 27 as char, termion::cursor::Goto(1, 1));
        }
        println!("N-puzzle: ");
        print_puzzle(&elem, final_state, opts);
        if opts.sleep {
            thread::sleep(time::Duration::from_millis(200));
        }
    }
    println!("###\n# {:?} heuristic", opts.heuristic);
    println!("# {} movements", len);
    println!("# {} states selected", close_list.len());
    println!(
        "# {} states represented in memory at the same time",
        nb_states
    );
}
