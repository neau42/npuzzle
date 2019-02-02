use std::io::ErrorKind as IoErr;
use std::io;
use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub struct Puzzle {
    pub size: i32,
    pub taq: Vec<u8>,
	pub actual_len: i32, //  G
	pub esimate_dst: i32,    //  H
	// dst:i32,
}

impl PartialEq for Puzzle {
		fn eq(&self, other: &Puzzle) -> bool {
			self.taq == other.taq
		}
	}

impl PartialOrd for Puzzle {
    fn partial_cmp(&self, other: &Puzzle) -> Option<Ordering> {
		(self.esimate_dst + self.actual_len).partial_cmp(&(other.esimate_dst + other.actual_len))
    }
}

impl Ord for Puzzle {
    fn cmp(&self, other: &Puzzle) -> Ordering {
		(self.esimate_dst + self.actual_len).cmp(&(other.esimate_dst + other.actual_len))
    }
}

impl Puzzle {
	pub fn copy(&self) -> Puzzle {
		let sq: usize = (self.size * self.size) as usize;
		let mut v: Vec<u8> = Vec::new();

		for i in 0..sq {
			v.push(self.taq[i]);
		}
		Puzzle { size: self.size as i32, taq : v, actual_len: self.actual_len, esimate_dst: self.esimate_dst }
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
				(true, false)  => index + 1,
				(true, true)   => index - 1,
				(false, false) => index + size as i32,
				(false, true)  => index - size as i32,
			};
			v[index as usize] = value as u8;
			cmpt -= 1;
		}
		Puzzle { size: size as i32, taq : v, actual_len: -1, esimate_dst: -1 }
	}

	pub fn get_pos_of_value(&self, value: u8) -> u8 {
		self.taq.iter().position(|r| *r == value).unwrap() as u8
	}

	fn get_pos_x_of_idx(&self, idx: u8) -> u8 {
		idx % self.size as u8
	}

	fn get_pos_y_of_idx(&self, idx: u8) -> u8 {
		 idx / self.size as u8
	}

	fn estimate_one(&self, final_state: &Puzzle, value: u8) -> u8 {
		let pos_current = self.get_pos_of_value(value);
		let pos_final = final_state.get_pos_of_value(value);

		((self.get_pos_x_of_idx(pos_current) as i32 - final_state.get_pos_x_of_idx(pos_final) as i32).abs()
			+ (self.get_pos_y_of_idx(pos_current) as i32 - final_state.get_pos_y_of_idx(pos_final) as i32).abs()) as u8
	}

	pub fn distance_estimator(&self, final_state: &Puzzle) -> u8 {
		let mut cmpt: u8 = 0;
		let sq: usize = (self.size * self.size) as usize;

		for i in 0..sq - 1 {
			cmpt += self.estimate_one(&final_state, i as u8);
		}
		cmpt
	}

	pub fn move_left(&self, zero_pos: usize) -> Result<Puzzle, io::Error> {
		if !(zero_pos % self.size as usize  == self.size as usize - 1) {
			let mut new_taquin = Puzzle{ size: self.size as i32, taq : self.taq.clone(), actual_len: -1, esimate_dst: -1 };
			new_taquin.taq.swap(zero_pos, zero_pos + 1);
			return Ok(new_taquin);
		}
		Err(std::io::Error::new(IoErr::Other, "_"))
	}

	pub fn move_right(&self, zero_pos: usize) -> Result<Puzzle, io::Error> {
		if !(zero_pos % self.size as usize  == 0) {
			let mut new_taquin = Puzzle{ size: self.size as i32, taq : self.taq.clone(), actual_len: -1, esimate_dst: -1 };
			new_taquin.taq.swap(zero_pos, zero_pos - 1);
			return Ok(new_taquin);
		}
		Err(std::io::Error::new(IoErr::Other, "_"))
	}

	pub fn move_down(&self, zero_pos: usize) -> Result<Puzzle, io::Error> {
		if !(zero_pos < self.size as usize) {
			let mut new_taquin = Puzzle{ size: self.size as i32, taq : self.taq.clone(), actual_len: -1, esimate_dst: -1 };
			new_taquin.taq.swap(zero_pos, zero_pos - self.size as usize);
			return Ok(new_taquin);
		}
		Err(std::io::Error::new(IoErr::Other, "_"))
	}

	pub fn move_up(&self, zero_pos: usize) -> Result<Puzzle, io::Error> {
		if !(zero_pos >= (self.size * self.size - self.size) as usize) {
			let mut new_taquin = Puzzle{ size: self.size as i32, taq : self.taq.clone(), actual_len: -1, esimate_dst: -1 };
			new_taquin.taq.swap(zero_pos, zero_pos + self.size as usize);
			return Ok(new_taquin);
		}
		Err(std::io::Error::new(IoErr::Other, "_"))
	}

	pub fn print(&self) {
		let sq: usize = (self.size * self.size) as usize;

		for i in 0..sq {
			print!("{number:>width$} ", number=self.taq[i], width=2);
			if i % (self.size as usize) == self.size as usize - 1 {
				print!("\n");
			}
		}
		println!("> H: ({}) G: ({}) F: ({})", self.esimate_dst, self.actual_len, self.actual_len + self.esimate_dst);
		print!("\n");
	}

	pub fn is_valid(&self) -> bool {
		let sq: u8 = (self.size * self.size) as u8;
		let mut v: Vec<u8> = Vec::new();

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

	pub fn is_soluble(&self) -> bool {
		let mut cmpt: u8 = 0;
		let mut vect_copy: Vec<u8> = self.taq.clone();
		let sq: usize = (self.size * self.size) as usize;
		let final_state = Puzzle::gen_final_state(self.size as usize);

		for idx in 0..sq - 1 {
			if vect_copy[idx as usize] != final_state.taq[idx as usize] {
				let pos = vect_copy.iter().enumerate().find(|r| *r.1 == final_state.taq[idx as usize]).unwrap().0;
				vect_copy.swap(idx as usize, (pos) as usize);
				cmpt += 1;
			}
		}
		(self.estimate_one(&final_state, 0) % 2 == cmpt % 2)
	}
}