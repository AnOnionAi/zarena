from gym_blackjack.gym_blackjack import BlackjackEngine  # rust module
from gym_blackjack.envs import BlackjackEnv  # envs
from gym.envs.registration import register  # to register envs

register(
    id="BlackjackR-v1",
    entry_point="gym_blackjack.envs:BlackjackEnv",
)
