from gym_gato.gym_gato import gatoEngine  # rust module
from gym_gato.envs import gatoEnv  # envs
from gym.envs.registration import register  # to register envs

register(
    id="gatoR-v2",
    entry_point="gym_gato.envs:gatoEnv",
)
