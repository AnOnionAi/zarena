from .envs.chess_env import ChessEnv  # envs
from gym.envs.registration import register  # to register envs

register(
    id="ChessVsRandomBot-v1",
    entry_point="gym_chess.envs:ChessEnvV1",
    kwargs={"opponent": "random"},
)

register(
    id="ChessVsSelf-v1",
    entry_point="gym_chess.envs:ChessEnvV1",
    kwargs={"opponent": "none"},
)

register(
    id="ChessVsRandomBot-v2",
    entry_point="gym_chess.envs:ChessEnvV2",
    kwargs={"opponent": "random"},
)

register(
    id="ChessVsSelf-v2",
    entry_point="gym_chess.envs:ChessEnvV2",
    kwargs={"opponent": "none"},
)

register(
    id="ChessVsRandomBot-v3",
    entry_point="gym_chess.envs:ChessEnvV3"
)

register(
    id="ChessVsSelf-v3",
    entry_point="zarena.gym_chess:ChessEnv"
)