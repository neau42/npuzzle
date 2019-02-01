
use std::env;
use std::process;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::{thread, time};
use rand::thread_rng;
use std::io;

// use std::io::Error;
use std::io::ErrorKind as IoErr;

use rand::seq::SliceRandom;
extern crate colored;
use colored::*;


// #[derive(Debug, Clone)]
#[derive(Debug)]
struct Taquin {
    size: i32,
    taq: Vec<u32>,
	actual_len: i32, //  G
	esimate_dst: i32,    //  H
	// dst:i32,
}

impl PartialEq for Taquin {
		fn eq(&self, other: &Taquin) -> bool {
			self.taq == other.taq
		}
	}

impl Taquin {
	fn gen_final_state(size: usize) -> Taquin {
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
			v[index as usize] = value as u32;
			cmpt -= 1;
		}
		Taquin { size: size as i32, taq : v.clone(), actual_len: -1, esimate_dst: -1 }
	}
	
	fn copy(&self) -> Taquin {
		let sq: usize = (self.size * self.size) as usize;
		let mut v: Vec<u32> = Vec::new();

		for i in 0..sq {
			// v[i] = self.taq[i];
			v.push(self.taq[i]);
		}
		Taquin { size: self.size as i32, taq : v, actual_len: self.actual_len, esimate_dst: self.esimate_dst }
	}

	fn get_pos_of_value(&self, value: u32) -> u32 {
		// self.taq.iter().enumerate().find(|r| *r.1 == value).unwrap().0 as u32
		self.taq.iter().position(|r| *r == value).unwrap() as u32
	}

	fn get_pos_x_of_idx(&self, idx: u32) -> u32 {
		idx % self.size as u32
	}

	fn get_pos_y_of_idx(&self, idx: u32) -> u32 {
		 idx / self.size as u32
	}

	fn estimate_one(&self, final_state: &Taquin, value: u32) -> u32 {
		let pos_current = self.get_pos_of_value(value);
		let pos_final = final_state.get_pos_of_value(value);

		((self.get_pos_x_of_idx(pos_current) as i32 - final_state.get_pos_x_of_idx(pos_final) as i32).abs()
			+ (self.get_pos_y_of_idx(pos_current) as i32 - final_state.get_pos_y_of_idx(pos_final) as i32).abs()) as u32
	}

	fn distance_estimator(&self, final_state: &Taquin) -> u32 {
		let mut cmpt: u32 = 0;
		let sq: usize = (self.size * self.size) as usize;

		for i in 0..sq - 1 {
			cmpt += self.estimate_one(&final_state, i as u32);
		}
		cmpt
	}

	fn move_left(&self, zero_pos: usize) -> Result<Taquin, io::Error> {
		if !(zero_pos % self.size as usize  == self.size as usize - 1) {
			let mut new_taquin = Taquin{ size: self.size as i32, taq : self.taq.clone(), actual_len: -1, esimate_dst: -1 };
			new_taquin.taq.swap(zero_pos, zero_pos + 1);
			return Ok(new_taquin);
		}
		Err(std::io::Error::new(IoErr::Other, "_"))
	}

	fn move_right(&self, zero_pos: usize) -> Result<Taquin, io::Error> {
		if !(zero_pos % self.size as usize  == 0) {
			let mut new_taquin = Taquin{ size: self.size as i32, taq : self.taq.clone(), actual_len: -1, esimate_dst: -1 };
			new_taquin.taq.swap(zero_pos, zero_pos - 1);
			return Ok(new_taquin);
		}
		Err(std::io::Error::new(IoErr::Other, "_"))
	}

	fn move_down(&self, zero_pos: usize) -> Result<Taquin, io::Error> {
		if !(zero_pos < self.size as usize) {
			let mut new_taquin = Taquin{ size: self.size as i32, taq : self.taq.clone(), actual_len: -1, esimate_dst: -1 };
			new_taquin.taq.swap(zero_pos, zero_pos - self.size as usize);
			return Ok(new_taquin);
		}
		Err(std::io::Error::new(IoErr::Other, "_"))
	}

	fn move_up(&self, zero_pos: usize) -> Result<Taquin, io::Error> {
		if !(zero_pos >= (self.size * self.size - self.size) as usize) {
			let mut new_taquin = Taquin{ size: self.size as i32, taq : self.taq.clone(), actual_len: -1, esimate_dst: -1 };
			new_taquin.taq.swap(zero_pos, zero_pos + self.size as usize);
			return Ok(new_taquin);
		}
		Err(std::io::Error::new(IoErr::Other, "_"))
	}

	fn print(&self) {
		let sq: usize = (self.size * self.size) as usize;

		for i in 0..sq {
			print!("{number:>width$} ", number=self.taq[i], width=2);
			// print!("{number:>2} ", number=self.taq[i]);
			if i % (self.size as usize) == self.size as usize - 1 {
				print!("\n");
			}
		}
		println!("> H: ({}) G: ({}) F: ({})", self.esimate_dst, self.actual_len, self.actual_len + self.esimate_dst);
		// println!("actual len: {} ", self.actual_len);
		print!("\n");
	}

	fn is_valid(&self) -> bool {
		let sq: u32 = (self.size * self.size) as u32;
		let mut v: Vec<u32> = Vec::new();

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

	fn is_soluble(&self) -> bool {
		let mut cmpt: u32 = 0;
		let mut vect_copy: Vec<u32> = self.taq.clone();
		let final_state = Taquin::gen_final_state(self.size as usize);

		for idx in 0..(self.size*self.size) - 1 {
			if vect_copy[idx as usize] != final_state.taq[idx as usize] {
				let pos = vect_copy.iter().enumerate().find(|r| *r.1 == final_state.taq[idx as usize]).unwrap().0;
				vect_copy.swap(idx as usize, (pos) as usize);
				cmpt += 1;
			}
		}
		(self.estimate_one(&final_state, 0) % 2 == cmpt % 2)
	}
}
fn create_open_list(puzzle: &mut Taquin, open_list: &mut Vec<Taquin>, close_list: &mut Vec<Taquin>, final_state: & Taquin) {
	// let functions: Vec< for<'r> fn(&'r Taquin) -> (Result<Taquin, io::Error>)> = vec![Taquin::move_down, Taquin::move_up, Taquin::move_left, Taquin::move_right];
	static MOVE_FUNCTIONS: &[ fn(&Taquin, usize) -> (Result<Taquin, io::Error>); 4]  = &[Taquin::move_down, Taquin::move_up, Taquin::move_left, Taquin::move_right];
	let zero_pos = puzzle.get_pos_of_value(0) as usize;
	// let final_state = Taquin::gen_final_state(puzzle.size as usize);

	for function in MOVE_FUNCTIONS {
			match function(&puzzle, zero_pos) {
			Ok(mut a) => if !close_list.contains(&a) {
				// if open_list.contains(&a) {
					// println!("open_list CONTAIN!");
// 
				// }
				a.esimate_dst = a.distance_estimator(&final_state) as i32;
				a.actual_len = puzzle.actual_len + 1;
				open_list.push(a);
			}
			Err(_) => (),
		}
	open_list.sort_by(|a, b| ((b.esimate_dst + b.actual_len).cmp(&(a.esimate_dst + a.actual_len))));
	}
}

fn solve(close_list: &mut Vec<Taquin>, open_list: &mut Vec<Taquin>) {
	let final_state = Taquin::gen_final_state(open_list[0].size as usize);

		// for _i in 0..8 {
		loop {
		let ref mut puzzle = open_list.pop().unwrap();
		// println!("current puzzle: ");
		// puzzle.print();
		if puzzle.esimate_dst == 0 {
			println!("SUCCESS -_-");
			puzzle.print();
			break ;
		}
		// println!("{}", "close list: ".green());
		// for e in close_list.iter() {
		// 	e.print();
		// }
		create_open_list(puzzle, open_list, close_list, &final_state);
		close_list.push(puzzle.copy());
		// println!("{}", "open list: ".green());
		// for e in open_list.iter() {
		// 	e.print();
		// }
		// println!("---------------------------- end");
		// thread::sleep(time::Duration::from_millis(350));
	}
}



fn get_puzzle(file_name: (bool, String)) -> Result<Taquin, io::Error> {
	match file_name.0 {
		true => read_file(file_name.1),
		false => Ok(generate_random_taquin(4)),
	}
}

// fn test_cp_vec(open_list: & Vec<Taquin>) -> Vec<Taquin> {
// 	let mut o_list: Vec<Taquin> = Vec::new();

// 	for e in open_list {
// 		o_list.push(e.copy());
// 	}
// 	o_list
// }

fn main() {
	let ref mut close_list: Vec<Taquin> = Vec::new();
	let ref mut open_list: Vec<Taquin> = Vec::new();

	let file_name = get_first_arg();
	let ref mut puzzle = match get_puzzle(file_name) {
		Ok(t) => t,
		Err(e) => {
			eprintln!("error: {}",e);
			return ;
		}
	};
	if puzzle.is_valid() && puzzle.is_soluble() {
		println!("VALID and Soluble!");
		puzzle.print();
		let final_state = Taquin::gen_final_state(puzzle.size as usize);
		puzzle.esimate_dst = puzzle.distance_estimator(&final_state) as i32;
		puzzle.actual_len = 0;
		// create_open_list(puzzle, open_list, close_list);
		open_list.push(puzzle.copy());


		// close_list.push(puzzle.copy());
		solve(close_list, open_list);
	}
	else {
		println!("NOT VALID");
	}
}

fn get_first_arg() -> (bool, String) {
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

fn read_file(name: String) -> Result<Taquin, io::Error> {
	let mut v: Vec<u32> = Vec::new();
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
				v.push(e.parse::<u32>().unwrap());
			}
		}
	}
	if size < 3 || v.len() != (size * size) as usize {
		return Err(std::io::Error::new(IoErr::Other, "not valid size"));
	}
	Ok(Taquin { size : size, taq : v.clone(), actual_len: -1, esimate_dst: -1 } )
}

fn generate_random_taquin(size: u32) -> Taquin {
	let mut rnd_taquin = Taquin::gen_final_state(size as usize);

	rnd_taquin.taq.shuffle(&mut thread_rng());
	rnd_taquin
}
			// 	match self.move_down() {
			// 	Ok(a) => match close_list.contains(&a) {
			// 		true  => (),
			// 		false => open_list.push(a),
			// 		}
			// 	Err(_) => (),
			// }
			// match self.move_up() {
			// 	Ok(a) => match close_list.contains(&a) {
			// 		true  => (),
			// 		false => open_list.push(a),
			// 		}
			// 	Err(_) => (),
			// }
			// match self.move_left() {
			// 	Ok(a) => match close_list.contains(&a) {
			// 		true  => (),
			// 		false => open_list.push(a),
			// 		}
			// 	Err(_) => (),
			// }
			// match self.move_right() {
			// 	Ok(a) => match close_list.contains(&a) {
			// 		true  => (),
			// 		false => open_list.push(a),
			// 		}
			// 	Err(_) => (),
			// }

