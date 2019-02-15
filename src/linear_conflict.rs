/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   linear_conflict.rs                                 :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/05 10:34:16 by no                #+#    #+#             */
/*   Updated: 2019/02/15 19:15:53 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::puzzle;

pub fn distance_estimator(taquin: &[u16], final_state: &puzzle::FinalPuzzle) -> i32 {
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
			// for i2 in taquin.iter().take(final_pos_y * size + size).skip(i + 1) {
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