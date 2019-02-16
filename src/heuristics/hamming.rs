/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   hamming.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/05 10:34:16 by no                #+#    #+#             */
/*   Updated: 2019/02/15 19:17:16 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::puzzle;

pub fn distance_estimator(taquin: &[u16], final_state: &puzzle::FinalPuzzle) -> i32 {
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