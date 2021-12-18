from zarena.zarena import TictactoeEngine  # rust module
from zarena.gym_tictactoe.envs.tictactoe import TictactoeEnv  # envs
from gym.envs.registration import register  # to register envs

register(
    id="TictactoeR-v2",
    entry_point="zarena.gym_tictactoe:TictactoeEnv",
)
