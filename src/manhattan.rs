/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   manhattan.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/15 19:08:20 by no                #+#    #+#             */
/*   Updated: 2019/02/15 19:21:27 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::puzzle;

pub fn estimate_one(
    final_state_posx: i32,
    final_state_posy: i32,
    taquin: &[u16],
    value: u16,
    size: i32,
) -> i32 {
    let pos_current = taquin.iter().position(|r| *r == value).unwrap() as i32;

    (((pos_current % size) - final_state_posx).abs()
        + ((pos_current / size) - final_state_posy).abs()) as i32
}

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
    }
    cmpt
}