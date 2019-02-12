/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/01 19:30:12 by no                #+#    #+#             */
/*   Updated: 2019/02/11 17:48:49 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod parser;
pub mod solver;
pub mod puzzle;
pub mod print;
pub mod heuristics;
pub mod options;

fn print_usage() {
	println!("usage:\tnpuzzle [-c -g -H -L -M -C -s] [file_name]");
	println!("\tc: color output");
	println!("\tg: greedy algo");
	println!("\tH: Heuristic: hamming");
	println!("\tL: Heuristic: Linear conflict");
	println!("\tM: Heuristic: Manhattan");
	println!("\tC: Heuristic: Manhattan + hamming (default)");
	println!("\ts: sleep to print output");
}

fn main() {

	let opts: options::Options = parser::get_arg();
	let mut puzzle = match parser::get_puzzle(&opts) {
		Ok(t) => t,
		Err(e) => {
			eprintln!("error: {}", e);
			print_usage();
			return ;
		}
	};
	if puzzle.is_valid() {
		let final_state: puzzle::FinalPuzzle = puzzle::init_final_stat(puzzle.size as usize);
		if puzzle.is_soluble(&final_state) {
			println!("puzzle solvable");
			solver::solve(&mut puzzle.taq, &final_state, &opts);
		} else {
			eprintln!("puzzle not solvable");
		}
	} else {
		eprintln!("not valid puzzle format");
	}
}