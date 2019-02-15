/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/01 19:30:12 by no                #+#    #+#             */
/*   Updated: 2019/02/13 18:46:44 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod heuristics;
pub mod options;
pub mod parser;
pub mod print;
pub mod puzzle;
pub mod solver;

fn print_usage() {
    println!("usage:\tnpuzzle [-c -g -H -L -M -C -s] [file_name]");
    println!("\t-c: color output");
    println!("\t-g: greedy algo");
    println!("\t-M: Heuristic: Manhattan");
    println!("\t-H: Heuristic: Hamming + Manhattan (Default)");
    println!("\t-L: Heuristic: Linear conflict + Manhattan");
    println!("\t-D: Djikstra");
    println!("\t-E: Euclidean");
    println!("\t-C: Chebyshev");
    println!("\t-s: sleep to print output");
}

fn main() {
    let opts: options::Options = parser::get_arg();
    let puzzle = match parser::get_puzzle(&opts) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error: {}", e);
            print_usage();
            return;
        }
    };
    if puzzle.is_valid() {
        let final_state: puzzle::FinalPuzzle = puzzle::init_final_stat(puzzle.size as usize);
        if puzzle.is_soluble(&final_state) {
            println!("puzzle solvable");
            solver::solve(puzzle.taq, &final_state, &opts);
        } else {
            eprintln!("puzzle not solvable");
        }
    } else {
        eprintln!("not valid puzzle format");
    }
}
