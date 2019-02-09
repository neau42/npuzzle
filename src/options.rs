/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   options.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: no <no@student.42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/06 10:28:13 by no                #+#    #+#             */
/*   Updated: 2019/02/06 15:23:14 by no               ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[derive(Debug)]
pub enum HeuristicType {
	Manhattan,
	Hamming,
	Linear,
	Combine,
}

#[derive(Debug)]
pub struct Options {
	pub file_name: String,
	pub file_name_present: bool,
	pub heuristic: HeuristicType,
	pub greedy: bool,
	pub color: bool,
	pub sleep: bool,
}

impl Options {
	pub fn new() -> Options {
	Options {
		file_name: "".to_string(),
		file_name_present: false,
		heuristic: HeuristicType::Combine,
		greedy: false,//?
		color: false,
		sleep: false }
	}
}