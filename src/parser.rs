/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parser.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/05 10:34:15 by no                #+#    #+#             */
/*   Updated: 2019/02/07 10:38:03 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::env;
use std::process;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::ErrorKind as IoErr;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::puzzle::Puzzle;
use crate::options::Options;
use crate::options::HeuristicType;

pub fn get_puzzle(opts: & Options) -> Result<Puzzle, io::Error> {
	match opts.file_name_present {
		true => read_file(&opts.file_name),
		false => Ok(generate_random_puzzle(3)),
	}
}

pub fn generate_random_puzzle(size: u32) -> Puzzle {
	let mut rnd_taquin = Puzzle::gen_final_state(size as usize);

	rnd_taquin.taq.shuffle(&mut thread_rng());
	rnd_taquin
}

fn get_opts(opts: &mut Options, s: &str) -> bool {
	match s {
		"-g" => opts.greedy = true,
		"-c" => opts.color = true,
		"-s" => opts.sleep = true,
		"-M" => opts.heuristic = HeuristicType::Manhattan,
		"-H" => opts.heuristic = HeuristicType::Hamming,
		"-L" => opts.heuristic = HeuristicType::Linear,
		"-C" => opts.heuristic = HeuristicType::Combine,
		_ => return false,
	}
	true
}

pub fn get_arg() -> Options {
	let args: Vec<String> = env::args().collect();
	let args: &[String] = &args[1..];
	let mut options = Options::new();
	let ref mut opts = options;

	
	for elem in args {
		if !get_opts(opts, &elem) {
			opts.file_name = elem.clone();
			opts.file_name_present = true;
			break ;
		}
	}
	options
}

fn get_size(line: String) -> Option<i32> {
	let mut size: Option<i32> = None;

	for e in line.split_whitespace() {
		if e.contains("#"){
			return size;
		}
		if size == None {
			let t = e.parse::<i32>();
			size = match t {
				Ok(x) => Some(x),
				_ => {
						println!("error unable to get n-puzzle size");
						process::exit(0);
				}
			}
		}
		else {
			return None;
		}
	}
	size
}

fn read_file(name: &String) -> Result<Puzzle, io::Error> {
	let mut v: Vec<u8> = Vec::new();
	let file = File::open(name)?;
	let mut size_opt = None;
	let mut size = 0;

	for line in BufReader::new(file).lines() {
		let line = match line {
			Ok(l) => {
			println!(">>> {}", l);
				l
			}
			Err(e) => return Err(e),
		};
		if size_opt == None {
			size_opt = get_size(line);
			size = size_opt.unwrap_or(0);
		}
		else {
			let mut count = 0;
			for e in line.split_whitespace() {
				if e.contains("#"){
					break ;
				}
				count += 1;
				if count > size || e.parse::<u32>().is_err() {
					return Err(std::io::Error::new(IoErr::Other, "Bad format"));
				}
				v.push(e.parse::<u8>().unwrap());
			}
		}
	}
	if size < 2 || size > 15 || v.len() != (size * size) as usize { // SIZE <= 15 || u8 -> u16 ;(
		return Err(std::io::Error::new(IoErr::Other, "not valid size"));
	}
	Ok(Puzzle { size : size, taq : v, actual_dst: 0, estimate_dst: -1 } )
}
