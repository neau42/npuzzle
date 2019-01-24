use std::env;
use std::process;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Taquin {
    size: i32,
    taq: Vec<u32>,
}

impl Taquin {
	fn gen_final_stat(size: usize) -> Taquin{
		let mut sq: usize = (size * size) as usize;
		let mut value: usize = 0;
		let mut idx: usize = 0;
		let mut v = vec![0; sq];
		// let mut size: usize = size as usize;


		while idx < size {
			value += 1;
			v[idx] = value as u32;
			println!("0>> {:?}", v);
			idx += 1;
		}
		idx = size * 2 - 1;
		while idx < sq {
			value += 1;
			v[idx] = value as u32;
			println!("1>> {:?}", v);
			idx += size;
		}
		idx = sq - 2;
		while (idx + 1) % (size) != 0 {
			value += 1;
			v[idx] = value as u32;
			println!("2>> {:?}", v);
			idx -= 1;
		}
		idx = sq - (size * 2);
		while idx >= size {
			value += 1;
			v[idx] = value as u32;
			println!("3>> {:?}", v);
			idx -= size;
		}
		println!("_>> {:?}", v);
		Taquin { size : size as i32, taq : v.clone() }
	}


	fn print(&self) {
		let sq: usize = (self.size * self.size) as usize;
		let mut i: usize = 0;

		while i < sq {
			print!("{} ", self.taq[i]);
			i += 1;
			if i % (self.size as usize) == 0 {
				println!("");
			}
		}
		println!("");

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
		true
	}

}

fn get_first_arg() -> String {
	let r: Vec<String> = env::args().collect();

	if r.len() != 2 {
		eprintln!("usage: {} [file_name]", &r[0]);
		process::exit(0);
	}
	r[1].clone()
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
	let mut size = -1;

	for line in BufReader::new(file).lines() {
	    let line = match line {
	        Ok(l) => l,
	        _ => panic!("coucou"),

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
	Taquin { size : size, taq : v.clone() }
}

fn main() {
	let file_name = get_first_arg();
	println!("valid file_name: {}", file_name);
	let init = read_file(file_name);

	if init.is_valid(){
	println!("VALID");
	let final_stat = Taquin::gen_final_stat(init.size as usize);
	init.print();
	final_stat.print();
	}
	else {
		println!("NOT VALID");
	}

	// println!("bla: {:?}", "42".to_string().parse::<i32>());
}
