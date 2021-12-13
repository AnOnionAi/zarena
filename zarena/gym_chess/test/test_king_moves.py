from copy import copy

import numpy as np
from zarena.gym_chess import ChessEnv
from zarena.gym_chess.envs.chess_env import (
    KING_ID,
    ROOK_ID,
    PAWN_ID,
)
from zarena.gym_chess.test.utils import run_test_funcs


# Blank board
BASIC_BOARD = np.array([[0] * 8] * 8, dtype=np.int8)
BASIC_BOARD[3, 3] = -PAWN_ID
BASIC_BOARD[2, 4] = -PAWN_ID
BASIC_BOARD[3, 5] = -PAWN_ID


# King capture movements
def test_king_moves_1():
    BOARD = copy(BASIC_BOARD)
    BOARD[4, 4] = KING_ID
    BOARD[0, 0] = ROOK_ID
    env = ChessEnv(initial_board=BOARD)
    env.reset()
    moves = env.get_possible_moves()
    env.render_moves(moves)
    env.render()
    king_is_checked = env.white_king_is_checked
    expected_attacks = set([(5, 5), (3, 4), (4, 3), (5, 4), (4, 5), (5, 3)])
    squares_attacked = set([tuple(move[1]) for move in moves])
    assert squares_attacked == expected_attacks
    assert king_is_checked


# King capture movements
def test_king_moves_2():
    BOARD = copy(BASIC_BOARD)
    BOARD[3, 4] = KING_ID
    env = ChessEnv(initial_board=BOARD)
    moves = env.get_possible_moves()
    env.render_moves(moves)
    king_is_checked = env.white_king_is_checked
    expected_attacks = set([(2, 4), (4, 3), (2, 3), (4, 5), (2, 5)])
    squares_attacked = set([tuple(move[1]) for move in moves])
    assert squares_attacked == expected_attacks
    assert not king_is_checked


if __name__ == "__main__":
    run_test_funcs(__name__)
