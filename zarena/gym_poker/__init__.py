from zarena.zarena import PokerEngine  # rust module
from zarena.gym_poker.envs.poker_env import PokerEnv  # envs
from gym.envs.registration import register  # to register envs

register(
    id="PokerR-v1",
    entry_point="zarena.gym_poker:PokerEnv",
)
