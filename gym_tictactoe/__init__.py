from gym_tictactoe.gym_tictactoe import TictactoeEngine  # rust module
from gym_tictactoe.envs import TictactoeEnv  # envs
from gym.envs.registration import register  # to register envs

register(
    id="TictactoeR-v2",
    entry_point="gym_tictactoe.envs:TictactoeEnv",
)
