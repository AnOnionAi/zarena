from zarena.zarena import ChessEngine  # rust module
from zarena.gym_chess.envs.chess_env import ChessEnv  # envs
from gym.envs.registration import register  # to register envs

register(id="ChessEnv-v3", entry_point="zarena.gym_chess:ChessEnv")
