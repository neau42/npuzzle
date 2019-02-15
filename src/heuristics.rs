/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   heuristics.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/05 10:34:16 by no                #+#    #+#             */
/*   Updated: 2019/02/15 19:19:47 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::options::HeuristicType;
use crate::options::Options;
use crate::puzzle;
use crate::manhattan;
use crate::hamming;
use crate::linear_conflict;
use crate::euclidean;
use crate::chebyshev;

pub fn distance_estimator(
    taquin:  &[u16],
    final_state: &puzzle::FinalPuzzle,
    opts: &Options,
) -> i32 {
    match opts.heuristic {
        HeuristicType::LinearConflict => {
            linear_conflict::distance_estimator(taquin, final_state)
            + manhattan::distance_estimator(taquin, final_state)
        }
        HeuristicType::Manhattan => manhattan::distance_estimator(taquin, final_state),
        HeuristicType::Hamming   => hamming::distance_estimator(taquin, final_state),
        HeuristicType::Euclidean => euclidean::distance_estimator(taquin, final_state),
        HeuristicType::Chebyshev => chebyshev::distance_estimator(taquin, final_state),
        HeuristicType::Djikstra  => 0,
    }
}
