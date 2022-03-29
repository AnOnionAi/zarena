from zarena.zarena import CheckersEngine  # rust module
from zarena.gym_checkers.envs.checkers import CheckersEnv  # envs
from gym.envs.registration import register  # to register envs

register(
    id="CheckersEnv-v1",
    entry_point="zarena.gym_checkers:CheckersEnv",
)
