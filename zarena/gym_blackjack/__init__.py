from .envs.blackjack_env import BlackjackEnv  # envs
from gym.envs.registration import register  # to register envs

register(
    id="BlackjackR-v1",
    entry_point="zarena.gym_blackjack:BlackjackEnv",
)
