from gym_poker.gym_poker import PokerEngine  # rust module
from gym_poker.envs import PokerEnv  # envs
from gym.envs.registration import register  # to register envs

register(
    id="PokerR-v1",
    entry_point="gym_poker.envs:PokerEnv",
)
