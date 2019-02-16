/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   chebyshev.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/05 10:34:16 by no                #+#    #+#             */
/*   Updated: 2019/02/15 19:19:06 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::puzzle;

pub fn estimate_one_chebyshev(
    final_state_posx: i32,
    final_state_posy: i32,
    taquin: &[u16],
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

pub fn distance_estimator(taquin: &[u16], final_state: &puzzle::FinalPuzzle) -> i32 {
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
