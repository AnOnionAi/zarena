import gym
from zarena.gym_blackjack import BlackjackEngine

# BLACKJACK GYM ENVIRONMENT CLASS
# ---------------------------
class BlackjackEnv(gym.Env):
    """
    Game wrapper.
    """

    def __init__(self, n_players=1):
        # engine
        self.engine = BlackjackEngine(n_players)

    def step(self, action):
        """
        Apply action to the game.

        Args:
            action : action of the action_space to take.

        Returns:
            The new observation, the reward and a boolean if the game has ended.
        """
        observation, reward, done = self.engine.step(action)
        return observation, reward, done, None

    def to_play(self):
        """
        Return the current player.

        Returns:
            The current player, it should be an element of the players list in the config.
        """
        return self.engine.to_play()

    def legal_actions(self):
        """
        Should return the legal actions at each turn, if it is not available, it can return
        the whole action space. At each turn, the game have to be able to handle one of returned actions.

        For complex game where calculating legal moves is too long, the idea is to define the legal actions
        equal to the action space but to return a negative reward if the action is illegal.

        Returns:
            An array of integers, subset of the action space.
        """
        return self.engine.legal_actions()

    def reset(self):
        """
        Reset the game for a new game.

        Returns:
            Initial observation of the game.
        """
        return self.engine.reset()

    def get_state(self):
        """
        Get the current state of the game
        """
        return self.engine.get_state()

    def get_total_players(self):
        """
        Returns:
            Integer total players + dealer
        """
        return self.engine.get_total_players()


# print("New Game")
# game = BlackjackEnv(1)
# game.reset()
# done = False
# bandera = ""
# while (not done):
#     game.render()
#     actions = ""
#     for i in range(0, len(game.legal_actions())):
#         actions += game.action_to_string(game.legal_actions()[i]) + ", "
#     bandera = input(actions + "\n")
#     _, reward, done = game.step(int(bandera))
#     print(reward, done)
# game.render()
# print(reward, done)
