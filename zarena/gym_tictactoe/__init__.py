from .envs.tictactoe_v2 import TictactoeEnv  # envs
from gym.envs.registration import register  # to register envs

register(
    id="TictactoeR-v2",
    entry_point="zarena.gym_tictactoe:TictactoeEnv",
)
