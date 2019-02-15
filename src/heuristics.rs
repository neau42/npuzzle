/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   heuristics.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/05 10:34:16 by no                #+#    #+#             */
/*   Updated: 2019/02/13 20:57:38 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::options::HeuristicType;
use crate::options::Options;
use crate::puzzle;

pub fn estimate_one_manhattan(
    final_state_posx: i32,
    final_state_posy: i32,
    taquin: &Vec<u16>,
    value: u16,
    size: i32,
) -> i32 {
    let pos_current = taquin.iter().position(|r| *r == value).unwrap() as i32;

    (((pos_current % size) - final_state_posx).abs()
        + ((pos_current / size) - final_state_posy).abs()) as i32
}

pub fn distance_estimator_manhattan(taquin: &Vec<u16>, final_state: &puzzle::FinalPuzzle) -> i32 {
    let mut cmpt: i32 = 0;
    let size = final_state.size;
    let sq: usize = (size * size) as usize;
    let mut final_pos;
    let mut current_pos;

    for i in 1..sq {
        final_pos = final_state.position[i as usize] as i32;
        current_pos = taquin.iter().position(|r| *r == i as u16).unwrap() as i32;
        cmpt += (((current_pos % size) - final_pos % size).abs()
            + ((current_pos / size) - final_pos / size).abs()) as i32;
    }
    cmpt
}

pub fn distance_estimator_linear(taquin: &Vec<u16>, final_state: &puzzle::FinalPuzzle) -> i32 {
    let size = final_state.size as usize;
    let sq: usize = size * size;

    let mut cmpt: i32 = 0;
    let mut current_pos_x: usize;
    let mut current_pos_y: usize;
    let mut final_pos_x: usize;
    let mut final_pos_y: usize;
    let mut current_value: usize;

    for i in 0..sq - 1 {
        current_value = taquin[i] as usize;
        if current_value == 0 || final_state.position[current_value] as usize == current_value {
            continue;
        }
        current_pos_x = i % size;
        current_pos_y = i / size;
        final_pos_x = final_state.position[current_value] as usize % size;
        final_pos_y = final_state.position[current_value] as usize / size;
        if current_pos_y == final_pos_y {
            //same line
            for i2 in (i + 1)..(final_pos_y * size + size) {
                if taquin[i2] != 0
                    && final_state.position[taquin[i2] as usize] as usize / size == current_pos_y
                    && final_state.position[taquin[i2] as usize] as usize % size < final_pos_x
                {
                    cmpt += 2;
                }
            }
        }
        if current_pos_x == final_pos_x {
            //same raw
            let mut i2 = i + size;
            while i2 < sq {
                if taquin[i2] != 0
                    && final_state.position[taquin[i2] as usize] as usize % size == current_pos_x
                    && final_state.position[taquin[i2] as usize] as usize / size < final_pos_y
                {
                    cmpt += 2;
                }
                i2 += size;
            }
        }
    }
    cmpt
}

pub fn distance_estimator_hamming(taquin: &Vec<u16>, final_state: &puzzle::FinalPuzzle) -> i32 {
    let mut cmpt: i32 = 0;
    let size = final_state.size;
    let sq: usize = (size * size) as usize;
    let mut final_pos;
    let mut current_pos;

    for i in 1..sq {
        final_pos = final_state.position[i as usize] as i32;
        current_pos = taquin.iter().position(|r| *r == i as u16).unwrap() as i32;
        cmpt += (((current_pos % size) - final_pos % size).abs()
            + ((current_pos / size) - final_pos / size).abs()) as i32;
        if taquin[i] != final_state.puzzle[i] {
            cmpt += 1;
        }
    }
    cmpt
}

pub fn estimate_one_euclidean(
    final_state_posx: i32,
    final_state_posy: i32,
    taquin: &Vec<u16>,
    value: u16,
    size: i32,
) -> i32 {
    let pos_current = taquin.iter().position(|r| *r == value).unwrap() as i32;

    ((((pos_current % size - final_state_posx) * (pos_current % size - final_state_posx))
        + (pos_current / size - final_state_posy) * (pos_current / size - final_state_posy))
        as f64)
        .sqrt() as i32
}

pub fn distance_estimator_euclidean(taquin: &Vec<u16>, final_state: &puzzle::FinalPuzzle) -> i32 {
    let mut cmpt: i32 = 0;
    let size = final_state.size;
    let sq: usize = (size * size) as usize;

    for i in 1..sq {
        cmpt += estimate_one_euclidean(
            (final_state.position[i as usize] as i32) % size,
            (final_state.position[i as usize] as i32) / size,
            taquin,
            i as u16,
            size,
        );
    }
    cmpt
}

pub fn estimate_one_chebyshev(
    final_state_posx: i32,
    final_state_posy: i32,
    taquin: &Vec<u16>,
    value: u16,
    size: i32,
) -> i32 {
    let pos_current = taquin.iter().position(|r| *r == value).unwrap() as i32;

    let dif_x = pos_current % size - final_state_posx;
    let dif_y = pos_current / size - final_state_posy;
    if dif_x > dif_y {
        return dif_x;
    }
    dif_y
}

pub fn distance_estimator_chebyshev(taquin: &Vec<u16>, final_state: &puzzle::FinalPuzzle) -> i32 {
    let mut cmpt: i32 = 0;
    let size = final_state.size;
    let sq: usize = (size * size) as usize;

    for i in 1..sq {
        cmpt += estimate_one_chebyshev(
            (final_state.position[i as usize] as i32) % size,
            (final_state.position[i as usize] as i32) / size,
            taquin,
            i as u16,
            size,
        );
    }
    cmpt
}
pub fn distance_estimator(
    taquin: &Vec<u16>,
    final_state: &puzzle::FinalPuzzle,
    opts: &Options,
) -> i32 {
    match opts.heuristic {
        HeuristicType::LinearConflict => {
            distance_estimator_linear(taquin, final_state)
                + distance_estimator_manhattan(taquin, final_state)
        }
        HeuristicType::Manhattan => distance_estimator_manhattan(taquin, final_state),
        HeuristicType::Hamming => distance_estimator_hamming(taquin, final_state),
        HeuristicType::Euclidean => distance_estimator_euclidean(taquin, final_state),
        HeuristicType::Chebyshev => distance_estimator_chebyshev(taquin, final_state),
        HeuristicType::Djikstra => 0,
    }
}
