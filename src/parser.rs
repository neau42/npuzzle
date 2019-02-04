use std::env;
use std::process;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::ErrorKind as IoErr;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::puzzle::Puzzle;

pub fn get_puzzle(file_name: (bool, String)) -> Result<Puzzle, io::Error> {
	match file_name.0 {
		true => read_file(file_name.1),
		false => Ok(generate_random_puzzle(3)),
	}
}

pub fn generate_random_puzzle(size: u32) -> Puzzle {
	let mut rnd_taquin = Puzzle::gen_final_state(size as usize);

	rnd_taquin.taq.shuffle(&mut thread_rng());
	rnd_taquin
}

pub fn get_arg() -> (bool, String) {
	let r: Vec<String> = env::args().collect();

	if r.len() > 2 {
		eprintln!("usage: {} [file_name]", &r[0]);
		process::exit(0);
	}
	if r.len() < 2 {
		return (false, "".to_string());
	}
	(true, r[1].clone())
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

fn read_file(name: String) -> Result<Puzzle, io::Error> {
	let mut v: Vec<u8> = Vec::new();
	let file = File::open(name)?;
	let mut size_opt = None;
	let mut size = 0;

	for line in BufReader::new(file).lines() {
		let line = match line {
			Ok(l) => l,
			Err(_) => return Err(std::io::Error::new(IoErr::Other, "unable to read file")),
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
	if size < 3 || v.len() != (size * size) as usize {
		return Err(std::io::Error::new(IoErr::Other, "not valid size"));
	}
	Ok(Puzzle { size : size, taq : v, actual_dst: -1, estimate_dst: -1 } )
}
