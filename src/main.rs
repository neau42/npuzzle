
use std::env;
use std::process;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::{thread, time};
use rand::thread_rng;
use rand::seq::SliceRandom;


#[derive(Debug)]
struct Taquin {
    size: i32,
    taq: Vec<u32>,
}

impl Taquin {
	fn gen_final_state(size: usize) -> Taquin {
		let sq: usize = size * size;
		let mut v = vec![0; sq];
		let mut horizontal: bool = true;
		let mut idx: i32 = -1;
		let mut inverse: bool = false;
		let mut compt: usize = size;
		let mut refcompt: usize = size;

		for value in 1..sq {
			if compt == 0 {
				inverse = (size % 2 == (refcompt - 1) % 2);
				horizontal = !horizontal;
				if !horizontal {
					refcompt -= 1;
				}
				compt = refcompt;
			}
			idx = match (horizontal, inverse) {
				(true, false) => idx + 1,
				(true, true) => idx - 1,
				(false, false) => idx + size as i32,
				(false, true) => idx - size as i32,
			};
			v[idx as usize] = value as u32;
			compt -= 1;
		}
		Taquin { size: size as i32, taq : v.clone() }
	}

	fn get_pos_of_value(&self, value: u32) -> u32 {
		self.taq.iter().enumerate().find(|r| *r.1 == value).unwrap().0 as u32
	}

	fn get_pos_x_of_idx(&self, idx: u32) -> u32 {
		idx % self.size as u32
	}

	fn get_pos_y_of_idx(&self, idx: u32) -> u32 {
		 idx / self.size as u32
	}

	fn move_left(&mut self) {
		let zero_pos = self.get_pos_of_value(0);
		if self.try_move_left() {
			self.taq.swap(zero_pos as usize, zero_pos as usize + 1);
		}
	}

	fn try_move_left(&self) -> bool {
		!(self.get_pos_of_value(0) % self.size as u32  == self.size as u32 - 1)
	}

	fn move_right(&mut self) {
		let zero_pos = self.get_pos_of_value(0);
		if self.try_move_right() {
			self.taq.swap(zero_pos as usize, zero_pos as usize - 1);
		}
	}

	fn calc_distance_for_zero(&self, value: u32) -> u32 {
		let mut taq_copy = Taquin { size: self.size, taq : self.taq.clone() };
		let mut pos = taq_copy.get_pos_of_value(value);
		let final_state = Taquin::gen_final_state(self.size as usize);
		let final_pos = final_state.get_pos_of_value(value);
		let mut cmpt: u32 = 0;

		while pos != final_pos {
			if taq_copy.get_pos_x_of_idx(pos) > final_state.get_pos_x_of_idx(final_pos) {
				taq_copy.move_right();
			}
			else if taq_copy.get_pos_x_of_idx(pos) < final_state.get_pos_x_of_idx(final_pos) {
				taq_copy.move_left();
			}
			else if taq_copy.get_pos_y_of_idx(pos) > final_state.get_pos_y_of_idx(final_pos) {
				taq_copy.move_down();
			}
			else {
				taq_copy.move_up();
			}
			pos = taq_copy.get_pos_of_value(value);
			cmpt += 1;
		}
		println!("distance for value {} (cmpt={}) init:{:?}, tmp:{:?} ", value, cmpt, self.taq, taq_copy.taq);
		cmpt
	}

	fn estimate_one(&self, final_state: &Taquin, value: u32) -> u32 {

		let pos_current = self.get_pos_of_value(value);
		let pos_final = final_state.get_pos_of_value(value);
		// println!("dif X {}", (self.get_pos_x_of_idx(pos_current) as i32 - final_state.get_pos_x_of_idx(pos_final) as i32).abs());
		// println!("dif Y {}", (self.get_pos_y_of_idx(pos_current) as i32 - final_state.get_pos_y_of_idx(pos_final) as i32).abs());
		((self.get_pos_x_of_idx(pos_current) as i32 - final_state.get_pos_x_of_idx(pos_final) as i32).abs() + (self.get_pos_y_of_idx(pos_current) as i32 - final_state.get_pos_y_of_idx(pos_final) as i32).abs()) as u32
	}


	fn distance_estimator(&self) -> u32 {
		let mut cmpt: u32 = 0;
		let mut tmp: u32;
		let final_state = Taquin::gen_final_state(self.size as usize);

		final_state.print();
		for i in 0..self.size*self.size - 1 {
			tmp = self.estimate_one(&final_state, i as u32);
			println!("estimate: {}", tmp);
			cmpt += tmp;
		}
		cmpt
	}

	fn try_move_right(&self) -> bool {
		!(self.get_pos_of_value(0) % self.size as u32  == 0)
	}

	fn move_down(&mut self) {
		let zero_pos = self.get_pos_of_value(0);
		if self.try_move_down() {
			self.taq.swap(zero_pos as usize, zero_pos as usize - self.size as usize);
		}
	}

	fn try_move_down(&self) -> bool {
		!(self.get_pos_of_value(0) < self.size as u32)
	}

	fn move_up(&mut self) {
		let zero_pos = self.get_pos_of_value(0);
		if self.try_move_up() {
			self.taq.swap(zero_pos as usize, zero_pos as usize + self.size as usize);
		}
	}

	fn try_move_up(&self) -> bool {
		!(self.get_pos_of_value(0) >= (self.size * self.size - self.size) as u32)
	}

	fn print(&self) {
		let sq: usize = (self.size * self.size) as usize;
		for i in 0..sq {
			print!("{number:>width$} ", number=self.taq[i], width=2);
			if i % (self.size as usize) == self.size as usize - 1 {
				print!("\n");
			}
		}
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
		// (self.calc_distance_for_zero(0) % 2 == cmpt % 2)
		(self.estimate_one(&final_state, 0) % 2 == cmpt % 2)
	}
	
	fn solve(&self) {
		let final_state = Taquin::gen_final_state(self.size as usize);

		// loop {
		
			println!("self({}): ", self.distance_estimator());
			self.print();
			println!("final_state({}): ", final_state.distance_estimator());

			final_state.print();
			thread::sleep(time::Duration::from_millis(250));
		// }


	}
}

fn get_first_arg() -> (bool, String) {
	let r: Vec<String> = env::args().collect();

	if r.len() > 2 {
		// 		let mut owned_string: String = "usage: ".to_owned();
		// owned_string.push_str(&r[0]);
		// owned_string.push_str(" [file_name]");
		// return (false, owned_string);
		eprintln!("usage: {} [file_name]", &r[0]);
		process::exit(0);
	}
	if r.len() < 2 {
		return (false, "".to_string());
	}
	(true, r[1].clone())
}

fn get_size(line: String) -> i32 {
	let mut size: i32 = -1;

	for e in line.split_whitespace() {
		if e.contains("#"){
			return size;
		}
		if size == -1 {
			size = e.parse::<i32>().expect("error unable to get n-puzzle size");
		}
		else {
			return -1;
		}
	}
	size
}

fn read_file(name: String) -> Taquin {
	let mut v: Vec<u32> = Vec::new();
	let file = File::open(name).expect("Failed to open file");
	// let mut size_ok = false;
	let mut size = -1;

	for line in BufReader::new(file).lines() {
		let line = match line {
			Ok(l) => l,
			_ => panic!("unable to read file"), //break!
		};
		if size == -1 {
			size = get_size(line);
		}
		else {
			let mut count = 0;
			for e in line.split_whitespace() {
				if e.contains("#"){
					break ;
				}
				count += 1;
				if count > size {
					panic!("bad format");
				}
				v.push(e.parse::<u32>().expect("unexpected value"));
			}
		}
	}
	if size < 3 {
		eprintln!("not valid size");
		process::exit(0);
	}
	Taquin { size : size, taq : v.clone() }
}

fn generate_random_taquin(size: u32) -> Taquin {
	let mut rnd_taquin = Taquin::gen_final_state(size as usize);

	// println!("{:?}", rnd_taquin.taq);
	rnd_taquin.taq.shuffle(&mut thread_rng());;
	rnd_taquin
}

fn main() {
	let file_name = get_first_arg();
	let mut taquin: Taquin;

	taquin = match file_name.0 {
		true => read_file(file_name.1),
		false => generate_random_taquin(4),
	};
	if taquin.is_valid() && taquin.is_soluble() {
		println!("VALID and Soluble!");
		taquin.print();
		taquin.solve();
		}
	else {
		println!("NOT VALID");
	}

	// println!("bla: {:?}", "42".to_string().parse::<i32>());
}

// fn main() {
//     let mut numbers = vec![41, 42, 43];
//     println!("before = {:?}", numbers);
//     numbers.swap(0, 2);
//     println!("after = {:?}", numbers);
// }