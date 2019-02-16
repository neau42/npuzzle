/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parser.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/05 10:34:15 by no                #+#    #+#             */
/*   Updated: 2019/02/15 19:05:50 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::options::HeuristicType;
use crate::options::Options;
use crate::puzzle::Puzzle;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env;
use std::fs::File;
use std::io;
use std::io::ErrorKind as IoErr;
use std::io::{BufRead, BufReader};
use std::process;

pub fn get_puzzle(opts: &Options) -> Result<Puzzle, io::Error> {
    if opts.file_name_present {
        read_file(&opts.file_name)
    } else {
        Ok(generate_random_puzzle(3))
    }
}

pub fn generate_random_puzzle(size: u16) -> Puzzle {
    let mut rnd_taquin = Puzzle::gen_final_state(size as usize);

    rnd_taquin.taq.shuffle(&mut thread_rng());
    rnd_taquin
}

fn get_opts(opts: &mut Options, s: &str) -> bool {
    match s {
        "-c" => opts.color = true,
        "-g" => opts.greedy = true,
        "-H" => opts.heuristic = HeuristicType::Hamming,
        "-L" => opts.heuristic = HeuristicType::LinearConflict,
        "-M" => opts.heuristic = HeuristicType::Manhattan,
        "-D" => opts.heuristic = HeuristicType::Djikstra,
        "-E" => opts.heuristic = HeuristicType::Euclidean,
        "-C" => opts.heuristic = HeuristicType::Chebyshev,
        "-s" => opts.sleep = true,
        _ => return false,
    }
    true
}

pub fn get_arg() -> Options {
    let args: Vec<String> = env::args().collect();
    let args: &[String] = &args[1..];
    let mut options = Options::new();
    let mut opts = &mut options;

    for elem in args {
        if !get_opts(opts, &elem) {
            opts.file_name = elem.clone();
            opts.file_name_present = true;
            break;
        }
    }
    options
}

fn get_size(line: String) -> Option<i32> {
    let mut size: Option<i32> = None;

    for e in line.split_whitespace() {
        if e.contains('#') {
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
        } else {
            return None;
        }
    }
    size
}

fn read_file(name: &str) -> Result<Puzzle, io::Error> {
    let file = File::open(name)?;
    let mut v: Vec<u16> = Vec::new();
    let mut size_opt = None;
    let mut size = 0;

    for line in BufReader::new(file).lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => return Err(e),
        };
        if size_opt == None {
            size_opt = get_size(line);
            size = size_opt.unwrap_or(0);
        } else {
            let mut count = 0;
            for e in line.split_whitespace() {
                if e.contains('#') {
                    break;
                }
                count += 1;
                if count > size || e.parse::<u16>().is_err() {
                    return Err(std::io::Error::new(IoErr::Other, "Bad format"));
                }
                v.push(e.parse::<u16>().unwrap());
            }
        }
    }
    if size < 2 || size > 17 || v.len() != (size * size) as usize {
        return Err(std::io::Error::new(IoErr::Other, "not valid size"));
    }
    Ok(Puzzle { size, taq: v })
}
