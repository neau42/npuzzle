use crate::puzzle::Puzzle;

pub fn estimate_one(taquin: & Vec<u8>, final_state: &Puzzle, value: u8, size: i32) -> i32 {
	let pos_current = taquin.iter().position(|r| *r == value).unwrap() as i32;
	let pos_final = final_state.get_pos_of_value(value) as i32;

	((((pos_current % size) - (pos_final % size)) as i32).abs()
		+ (((pos_current / size) - (pos_final / size)) as i32).abs()) as i32
}


pub fn distance_estimator_manhattan(taquin: & Vec<u8>, final_state: &Puzzle) -> i32 {
	let mut cmpt: i32 = 0;
	let size = final_state.size;
	let sq: usize = (size) as usize;

	for i in 0..sq - 1 {
		cmpt += estimate_one(taquin, &final_state, i as u8, size);
	}
	cmpt
}

pub fn distance_estimator_hamming(taquin: & Vec<u8>, final_state: &Puzzle) -> i32 {
	let mut cmpt: i32 = 0;
	let sq: usize = (final_state.size * final_state.size) as usize;

	for i in 0..sq - 1 {
		if taquin[i] != final_state.taq[i] {
			cmpt += 1;
		}
	}
	cmpt
}

pub fn distance_estimator(taquin: & Vec<u8>, final_state: &Puzzle) -> i32 {
	let mut cmpt: i32 = 0;
	let size = final_state.size;
	let sq: usize = (size * size) as usize - 1;

	for i in 0..sq {
		cmpt += estimate_one(taquin, &final_state, i as u8, size);
		if taquin[i] != final_state.taq[i] {
			cmpt += 1;
		}
	}
	cmpt
}
