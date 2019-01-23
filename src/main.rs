use std::env;
use std::process;

fn get_first_arg() -> String {
	let r: Vec<String> = env::args().collect();
	if r.len() != 2 {
		eprintln!("usage: {} [file_name]", &r[0]);
		process::exit(0);
	}
	r[1].clone()
}

fn read_file(name: String) -> Vec<u32> {
	 vec![42]
}

fn main() {
	let file_name = get_first_arg();
	println!("valid file_name: {}", file_name);
	let value = read_file(file_name);
	println!("value: {:?}", value);

}
