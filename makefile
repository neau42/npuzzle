# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: nboulaye <nboulaye@student.42.fr>          +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2019/02/16 21:55:43 by nboulaye          #+#    #+#              #
#    Updated: 2019/02/16 21:55:43 by nboulaye         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #


NAME = npuzzle

SRC = src/heuristics.rs\
	src/heuristics/chebyshev.rs\
	src/heuristics/euclidean.rs\
	src/heuristics/hamming.rs\
	src/heuristics/linear_conflict.rs\
	src/heuristics/manhattan.rs\
	src/main.rs\
	src/options.rs\
	src/parser.rs\
	src/print.rs\
	src/puzzle.rs\
	src/solver.rs

all: $(NAME)

$(NAME): $(SRC)
	@cargo build --release
	@cp target/release/npuzzle .

clean:
	cargo clean

fclean: clean
	@rm -f npuzzle

re: fclean all

.PHONY: all clean fclean re
