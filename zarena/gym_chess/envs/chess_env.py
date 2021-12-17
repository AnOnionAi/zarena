import os
import sys
import datetime
import numpy
import toml
import gym

from collections import defaultdict
from copy import copy
from dataclasses import dataclass
from six import StringIO
from pprint import pprint
from gym import spaces, error, utils
from gym.utils import seeding

from zarena.gym_chess import ChessEngine

EMPTY_SQUARE_ID = 0
KING_ID = 1
QUEEN_ID = 2
ROOK_ID = 3
BISHOP_ID = 4
KNIGHT_ID = 5
PAWN_ID = 6

KING = "king"
QUEEN = "queen"
ROOK = "rook"
BISHOP = "bishop"
KNIGHT = "knight"
PAWN = "pawn"

KING_DESC = "K"
QUEEN_DESC = "Q"
ROOK_DESC = "R"
BISHOP_DESC = "B"
KNIGHT_DESC = "N"
PAWN_DESC = ""

WHITE_ID = 1
BLACK_ID = -1

WHITE = "WHITE"
BLACK = "BLACK"

WIN_REWARD = 1
LOSS_REWARD = 0
DRAW_REWARD = 0.5
INVALID_ACTION_REWARD = -1


@dataclass
class Piece:
    id: int
    icon: str
    desc: str
    type: str
    color: str


PIECES = [
    Piece(icon="♙", desc=PAWN_DESC, color=BLACK, type=PAWN, id=-PAWN_ID),
    Piece(icon="♘", desc=KNIGHT_DESC, color=BLACK, type=KNIGHT, id=-KNIGHT_ID),
    Piece(icon="♗", desc=BISHOP_DESC, color=BLACK, type=BISHOP, id=-BISHOP_ID),
    Piece(icon="♖", desc=ROOK_DESC, color=BLACK, type=ROOK, id=-ROOK_ID),
    Piece(icon="♕", desc=QUEEN_DESC, color=BLACK, type=QUEEN, id=-QUEEN_ID),
    Piece(icon="♔", desc=KING_DESC, color=BLACK, type=KING, id=-KING_ID),
    Piece(icon=".", desc="", color=None, type=None, id=EMPTY_SQUARE_ID),
    Piece(icon="♚", desc=KING_DESC, color=WHITE, type=KING, id=KING_ID),
    Piece(icon="♛", desc=QUEEN_DESC, color=WHITE, type=QUEEN, id=QUEEN_ID),
    Piece(icon="♜", desc=ROOK_DESC, color=WHITE, type=ROOK, id=ROOK_ID),
    Piece(icon="♝", desc=BISHOP_DESC, color=WHITE, type=BISHOP, id=BISHOP_ID),
    Piece(icon="♞", desc=KNIGHT_DESC, color=WHITE, type=KNIGHT, id=KNIGHT_ID),
    Piece(icon="♟", desc=PAWN_DESC, color=WHITE, type=PAWN, id=PAWN_ID),
]

ID_TO_COLOR = {piece.id: piece.color for piece in PIECES}
ID_TO_ICON = {piece.id: piece.icon for piece in PIECES}
ID_TO_TYPE = {piece.id: piece.type for piece in PIECES}
ID_TO_DESC = {piece.id: piece.desc for piece in PIECES}

RESIGN = "RESIGN"
CASTLE_KING_SIDE_WHITE = "CASTLE_KING_SIDE_WHITE"
CASTLE_QUEEN_SIDE_WHITE = "CASTLE_QUEEN_SIDE_WHITE"
CASTLE_KING_SIDE_BLACK = "CASTLE_KING_SIDE_BLACK"
CASTLE_QUEEN_SIDE_BLACK = "CASTLE_QUEEN_SIDE_BLACK"
CASTLE_MOVES = [
    CASTLE_KING_SIDE_WHITE,
    CASTLE_QUEEN_SIDE_WHITE,
    CASTLE_KING_SIDE_BLACK,
    CASTLE_QUEEN_SIDE_BLACK,
]

DEFAULT_BOARD = [
    [-3, -5, -4, -2, -1, -4, -5, -3],
    [-6, -6, -6, -6, -6, -6, -6, -6],
    [0] * 8,
    [0] * 8,
    [0] * 8,
    [0] * 8,
    [6, 6, 6, 6, 6, 6, 6, 6],
    [3, 5, 4, 2, 1, 4, 5, 3],
]

FILE_NAMES = ["a", "b", "c", "d", "e", "f", "g", "h"]
RANK_NAMES = ["1", "2", "3", "4", "5", "6", "7", "8"]


def highlight(string, background="white", color="gray"):
    return utils.colorize(utils.colorize(string, color), background, highlight=True)


# CHESS GYM ENVIRONMENT CLASS
# ---------------------------
class ChessEnv(gym.Env):
    def __init__(self, player_color=WHITE, log=True, initial_board=DEFAULT_BOARD):

        # constants
        self.log = log
        self.initial_board = initial_board
        self.board = self.initial_board

        # engine
        self.engine = ChessEngine()

        # Muzero control of players
        self.player = 1
        self.player_color = player_color  # define player # TODO: implement
        self.current_player = player_color
        self.white_king_castle_is_possible = True
        self.white_queen_castle_is_possible = True
        self.black_king_castle_is_possible = True
        self.black_queen_castle_is_possible = True
        self.white_king_is_checked = False
        self.black_king_is_checked = False

    def seed(self, seed=None):
        self.np_random, seed = seeding.np_random(seed)

        return [seed]

    def to_play(self):
        return 0 if self.player == 1 else 1

    def reset(self):
        """
        Resets the state of the environment, returning an initial observation.
        Outputs -> observation : the initial observation of the space. (Initial reward is assumed to be 0.)
        """
        self.board = self.initial_board
        # self.prev_board = None ## TODO: use prev_board to check for en-passant mvoes
        self.done = False
        self.player = 1
        self.current_player = WHITE
        self.saved_boards = defaultdict(lambda: 0)
        self.repetitions = 0  # 3 repetitions ==> DRAW
        self.states = []
        self.move_count = 0
        self.white_king_castle_is_possible = True
        self.white_queen_castle_is_possible = True
        self.black_king_castle_is_possible = True
        self.black_queen_castle_is_possible = True
        self.white_king_is_checked = False
        self.black_king_is_checked = False
        self.white_king_on_the_board = self.piece_is_on_board(self.board, KING_ID)
        self.black_king_on_the_board = self.piece_is_on_board(self.board, -KING_ID)
        # update state with engine
        self.state = self.engine.update_state(self.state)
        # pre-calculate possible moves
        self.possible_moves = self.get_possible_moves(state=self.state, player=WHITE)
        # If player chooses black, make white opponnent move first
        if self.player == BLACK:
            white_first_move = self.opponent_policy(self)
            white_first_action = self.move_to_action(white_first_move)
            # make move
            # self.board, _, _, _ = self.step(white_first_action)
            self.state, _, _ = self.player_move(white_first_action)
            self.move_count += 1
            self.current_player = BLACK
            self.possible_moves = self.get_possible_moves(
                state=self.state, player=BLACK
            )

        return self.get_observation()

    def step(self, action):
        """
        Run one timestep of the environment's dynamics. When end of episode
        is reached, reset() should be called to reset the environment's internal state.

        Input
        -----
        action : an action provided by the environment

        Outputs
        -------
        (observation, reward, done, info)
        observation : agent's observation of the current environment
        reward [Float] : amount of reward due to the previous action
        done : a boolean, indicating whether the episode has ended
        info : a dictionary containing other diagnostic information from the previous action
        """

        # action invalid in current state
        self.possible_moves = self.get_possible_moves()
        if action not in self.possible_actions:
            return self.get_observation, INVALID_ACTION_REWARD, self.done, self.info

        # Game is done
        if self.done:
            return self.get_observation, DRAW_REWARD, True, self.info

        # make move
        self.state, reward = self.player_move(action)

        # 3-fold repetition => DRAW
        self.states.append(self.state)
        if self.engine.is_game_over(self.states, self.state, self.current_player) != 0:
            self.done = True

        if self.have_winner() or len(self.legal_actions(self.switch_player())) == 0:
            self.done = True
        reward = self.get_reward() if self.done else 0

        if self.done:
            return self.get_observation(), reward, True, self.info

        # Change Player Color
        self.current_player = self.switch_player()

        # Change player Number
        self.player *= -1

        if self.current_player == WHITE:
            self.move_count += 1

            print("# of turns: " + str(self.move_count))

        return self.get_observation(), reward, self.done, self.info

    def have_winner(self):

        # check if there are no possible_moves for self
        # AKA lose
        if not self.possible_moves and self.king_is_checked(player=self.current_player):
            return True

        # AKA WIN
        opponent_player = self.switch_player()
        self.possible_moves = self.get_possible_moves(player=opponent_player)
        # check if there are no possible_moves for opponent
        if not self.possible_moves and self.king_is_checked(player=opponent_player):
            return True

        return False

    def get_reward(self):
        # check if there are no possible_moves for self
        # LOSE
        # if not self.possible_moves and self.king_is_checked(player=self.current_player):
        #     return LOSS_REWARD

        # WIN
        # opponent_player = self.switch_player()
        # self.possible_moves = self.get_possible_moves(player=opponent_player)
        # check if there are no possible_moves for opponent
        # if not self.possible_moves and self.king_is_checked(player=opponent_player):
        #     return WIN_REWARD
        player = self.current_player
        other_player = self.switch_player()
        state = self.state
        states = self.states
        # call Rust binary
        if self.engine.is_game_over(states, state, player) == 1:
            return LOSS_REWARD
        if self.engine.is_game_over(states, state, other_player) == 1:
            return WIN_REWARD
        return DRAW_REWARD

    def switch_player(self):
        other_player = self.get_other_player(self.current_player)
        return other_player

    @property
    def state(self):
        return dict(
            board=self.board,
            # prev_board=self.prev_board,
            current_player=self.current_player,
            white_king_castle_is_possible=self.white_king_castle_is_possible,
            white_queen_castle_is_possible=self.white_queen_castle_is_possible,
            black_king_castle_is_possible=self.black_king_castle_is_possible,
            black_queen_castle_is_possible=self.black_queen_castle_is_possible,
            white_king_is_checked=self.white_king_is_checked,
            black_king_is_checked=self.black_king_is_checked,
        )

    @state.setter
    def state(self, state):
        self.board = state.get("board")
        self.white_king_castle_is_possible = state.get("white_king_castle_is_possible")
        self.white_queen_castle_is_possible = state.get(
            "white_queen_castle_is_possible"
        )
        self.black_king_castle_is_possible = state.get("black_king_castle_is_possible")
        self.black_queen_castle_is_possible = state.get(
            "black_queen_castle_is_possible"
        )
        self.white_king_is_checked = state.get("white_king_is_checked")
        self.black_king_is_checked = state.get("black_king_is_checked")

    @property
    def possible_moves(self):
        return self._possible_moves

    @possible_moves.setter
    def possible_moves(self, moves):
        self._possible_moves = moves

    @property
    def possible_actions(self):
        return [self.move_to_action(m) for m in self.possible_moves]

    @property
    def info(self):
        return dict(
            # **self.state,
            move_count=self.move_count,
            current_player=self.current_player,
            possible_moves=self.possible_moves,
            # possible_actions=self.possible_actions,
            white_king_castle_is_possible=self.white_king_castle_is_possible,
            white_queen_castle_is_possible=self.white_queen_castle_is_possible,
            black_king_castle_is_possible=self.black_king_castle_is_possible,
            black_queen_castle_is_possible=self.black_queen_castle_is_possible,
            white_king_is_checked=self.white_king_is_checked,
            black_king_is_checked=self.black_king_is_checked,
            white_king_on_the_board=self.white_king_on_the_board,
            black_king_on_the_board=self.black_king_on_the_board,
        )

    @property
    def opponent_player(self):
        if self.current_player == WHITE:
            return BLACK
        return WHITE

    @property
    def current_player_is_white(self):
        return self.current_player == WHITE

    @property
    def current_player_is_black(self):
        return not self.current_player_is_white

    def king_is_checked(self, player):
        if player == WHITE:
            return self.white_king_is_checked
        else:
            return self.black_king_is_checked

    def is_checkmate(self):
        if not self.king_is_checked():
            return False

    def piece_is_on_board(self, board, piece_id):
        for row in board:
            for square in row:
                if square == piece_id:
                    return True
        return False

    def player_can_castle(self, player):
        if player == WHITE:
            return (
                self.white_king_castle_is_possible
                and self.white_queen_castle_is_possible
            )
        else:
            return (
                self.black_king_castle_is_possible
                and self.black_queen_castle_is_possible
            )

    def get_other_player(self, player):
        if player == WHITE:
            return BLACK
        return WHITE

    def player_move(self, action):
        """
        Returns (state, reward, done)
        """
        # Resign
        if self.is_resignation(action):
            return self.state, LOSS_REWARD
        # Play
        move = self.action_to_move(action)
        new_state = self.next_state(self.state, self.current_player, move)
        # Render
        if self.log:
            print(" " * 10, ">" * 10, self.current_player)
            self.render_moves([move], mode="human")
        return new_state, LOSS_REWARD

    def next_state(self, state, player, move):
        if state is None:
            state = self.state
        move_str = self.move_to_str_code(move)
        # breakpoint()
        next_state = self.engine.next_state(state, player, move_str)
        return next_state

    def board_to_grid(self):
        grid = [[f" {ID_TO_ICON[square]} " for square in row] for row in self.board]
        return grid

    def render_grid(self, grid, mode="human"):
        outfile = sys.stdout if mode == "human" else StringIO()
        outfile.write("    ")
        outfile.write("-" * 25)
        outfile.write("\n")
        rows = "87654321"
        for i, row in enumerate(grid):
            outfile.write(f" {rows[i]} | ")
            for square in row:
                outfile.write(square)
            outfile.write("|\n")
        outfile.write("    ")
        outfile.write("-" * 25)
        outfile.write("\n      a  b  c  d  e  f  g  h ")
        outfile.write("\n")

        if mode == "string":
            return outfile.getvalue()
        if mode != "human":
            return outfile

    def render(self, mode="human"):
        """Render the playing board"""
        grid = self.board_to_grid()
        out = self.render_grid(grid, mode=mode)
        return out

    def render_moves(self, moves, mode="human"):
        grid = self.board_to_grid()
        for move in moves:
            if type(move) is str and move in CASTLE_MOVES:
                if move == CASTLE_QUEEN_SIDE_WHITE:
                    grid[7][0] = highlight(grid[7][0], background="white")
                    grid[7][1] = highlight(" >>", background="green")
                    grid[7][2] = highlight("> <", background="green")
                    grid[7][3] = highlight("<< ", background="green")
                    grid[7][4] = highlight(grid[7][4], background="white")
                elif move == CASTLE_KING_SIDE_WHITE:
                    grid[7][4] = highlight(grid[7][4], background="white")
                    grid[7][5] = highlight(" >>", background="green")
                    grid[7][6] = highlight("<< ", background="green")
                    grid[7][7] = highlight(grid[7][7], background="white")
                elif move == CASTLE_QUEEN_SIDE_BLACK:
                    grid[0][0] = highlight(grid[0][0], background="white")
                    grid[0][1] = highlight(" >>", background="green")
                    grid[0][2] = highlight("> <", background="green")
                    grid[0][3] = highlight("<< ", background="green")
                    grid[0][4] = highlight(grid[0][4], background="white")
                elif move == CASTLE_KING_SIDE_BLACK:
                    grid[0][4] = highlight(grid[0][4], background="white")
                    grid[0][5] = highlight(" >>", background="green")
                    grid[0][6] = highlight("<< ", background="green")
                    grid[0][7] = highlight(grid[0][7], background="white")
                continue

            x0, y0 = move[0][0], move[0][1]
            x1, y1 = move[1][0], move[1][1]
            if len(grid[x0][y0]) < 4:
                grid[x0][y0] = highlight(grid[x0][y0], background="white")
            if len(grid[x1][y1]) < 4:
                if self.board[x1][y1]:
                    grid[x1][y1] = highlight(grid[x1][y1], background="red")
                else:
                    grid[x1][y1] = highlight(grid[x1][y1], background="green")
        return self.render_grid(grid, mode=mode)

    def move_to_action(self, move):
        if type(move) in [list, tuple]:
            _from = move[0][0] * 8 + move[0][1]
            _to = move[1][0] * 8 + move[1][1]
            return _from * 64 + _to
        if move == CASTLE_KING_SIDE_WHITE:
            return 64 * 64
        elif move == CASTLE_QUEEN_SIDE_WHITE:
            return 64 * 64 + 1
        elif move == CASTLE_KING_SIDE_BLACK:
            return 64 * 64 + 2
        elif move == CASTLE_QUEEN_SIDE_BLACK:
            return 64 * 64 + 3
        elif move == RESIGN:
            return 64 * 64 + 4

    def action_to_move(self, action):
        return self._action_to_move(action, as_string=False)

    def action_to_move_str(self, action):
        return self._action_to_move(action, as_string=True)

    def _action_to_move(self, action, as_string=False):
        if action >= 64 * 64:
            _action = action - 64 * 64
            if _action == 0:
                return CASTLE_KING_SIDE_WHITE
            elif _action == 1:
                return CASTLE_QUEEN_SIDE_WHITE
            elif _action == 2:
                return CASTLE_KING_SIDE_BLACK
            elif _action == 3:
                return CASTLE_QUEEN_SIDE_BLACK
            elif _action == 4:
                return RESIGN
        _from, _to = action // 64, action % 64
        x0, y0 = _from // 8, _from % 8
        x1, y1 = _to // 8, _to % 8
        if not as_string:
            return ((x0, y0), (x1, y1))
        return self.move_to_str_code(action)

    def move_to_str_code(self, move):
        if move in CASTLE_MOVES:
            return move
        (x0, y0), (x1, y1) = move
        rows = list(reversed("12345678"))
        cols = "abcdefgh"
        return f"{cols[y0]}{rows[x0]}{cols[y1]}{rows[x1]}"

    def move_to_string(self, move):
        if move in [CASTLE_KING_SIDE_WHITE, CASTLE_KING_SIDE_BLACK]:
            return "O-O"
        elif move in [CASTLE_QUEEN_SIDE_WHITE, CASTLE_QUEEN_SIDE_BLACK]:
            return "O-O-O"
        _from, _to = move
        rows = list(reversed("12345678"))
        cols = "abcdefgh"
        piece_id = self.board[_from[0]][_from[1]]
        piece_desc = ID_TO_DESC[piece_id]
        capture = self.board[_to[0]][_to[1]] != 0
        _from_str = cols[_from[1]] + rows[_from[0]]
        _to_str = cols[_to[1]] + rows[_to[0]]
        string = f"{piece_desc}{_from_str}{'x' if capture else ''}{_to_str}"
        return string

    def rust_move_to_coords(self, move):
        if move in CASTLE_MOVES:
            return move
        cols = dict(a=0, b=1, c=2, d=3, e=4, f=5, g=6, h=7)
        _from = move[:2]
        _to = move[2:]
        _from = (8 - int(_from[1]), cols[_from[0]])
        _to = (8 - int(_to[1]), cols[_to[0]])
        move = (_from, _to)
        return move

    def get_possible_actions(self, player):
        moves = self.get_possible_moves(None, player)
        return [self.move_to_action(move) for move in moves]

    def get_possible_moves(self, state=None, player=None, attack=False):
        if state is None:
            state = self.state
        if player is None:
            player = self.current_player
        # call Rust binary
        moves = self.engine.get_possible_moves(state, player, attack)
        # print(moves)
        moves = [self.rust_move_to_coords(move) for move in moves]
        return moves

    def get_castle_moves(self, state=None, player=None):
        if state is None:
            state = self.state
        if player is None:
            player = self.current_player
        # call Rust binary
        moves = self.engine.get_castle_moves(state, player)
        # print(moves)
        moves = [self.rust_move_to_coords(move) for move in moves]
        return moves

    # TODO: implement resignation action parsing
    def is_resignation(self, action):
        return False

    @staticmethod
    def player_to_int(player):
        if player == WHITE:
            return 1
        return -1

    def encode_board(self):
        mapping = "0ABCDEFfedcba"
        encoding = "".join([mapping[val] for row in self.board for val in row])
        return encoding

    def get_observation(self):
        # TODO - Instead of entire state, player1 and player2
        # board_player1=numpy.where(self.board == White)
        # board_player2=numpy.where(self.board == Black)
        return [
            # TODO - Only Player 1
            self.board,
            # Only to Player 2
            self.board,
        ]

    def legal_actions(self, player=None):
        if player == None:
            player = self.current_player
        # Not all possible actions are legal but good enough for now
        return self.get_possible_actions(player)

    def close(self):
        pass


# import random
# game = ChessEnvV3()
# reward = []
# for i in range(0, 10):
#     done = False
#     game.reset()
#     reward_3 = 0
#     while not done:
#         # print(game.legal_actions())
#         action = random.choice(game.legal_actions())
#         _, reward_2, done, _ = game.step(action)
#         reward_3 = reward_2
#     reward.append(reward_2)
#     print(f"Reward: {reward}")
