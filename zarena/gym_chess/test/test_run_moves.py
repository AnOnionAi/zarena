from copy import copy

import numpy as np
from zarena.gym_chess import ChessEnv
from zarena.gym_chess.envs.chess_env import PAWN_ID, DEFAULT_BOARD
from zarena.gym_chess.test.utils import run_test_funcs


# Blank board
BASIC_BOARD = np.array(DEFAULT_BOARD)

# Pawn basic movements
def test_pawn_basic_moves():
    BOARD = copy(BASIC_BOARD)
    BOARD[6, 0] = PAWN_ID
    BOARD[1, 0] = -PAWN_ID
    env = ChessEnv(initial_board=BOARD)
    env.reset()
    # player_1
    actions = env.legal_actions()
    env.step(actions[0])
    # player_2
    actions = env.legal_actions()
    env.step(actions[-1])
    # player_3
    actions = env.legal_actions()
    env.step(actions[0])
    # player_4
    actions = env.legal_actions()
    env.step(actions[-1])
    env.render()

    EXPECTED_BOARD = copy(BASIC_BOARD)
    EXPECTED_BOARD[6, 0] = 0
    EXPECTED_BOARD[4, 0] = PAWN_ID
    EXPECTED_BOARD[1, 7] = 0
    EXPECTED_BOARD[4, 7] = -PAWN_ID
    print(np.array(EXPECTED_BOARD))
    assert (np.array(env.state["board"]) == np.array(EXPECTED_BOARD)).all()


if __name__ == "__main__":
    run_test_funcs(__name__)
