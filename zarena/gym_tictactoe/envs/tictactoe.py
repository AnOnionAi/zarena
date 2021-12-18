import gym
from zarena.gym_tictactoe import TictactoeEngine

# TICTACTOE GYM ENVIRONMENT CLASS
# ---------------------------
class TictactoeEnv(gym.Env):
    """
    Game wrapper.
    """

    def __init__(self, n_players=1):
        # engine
        self.engine = TictactoeEngine(n_players)

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

    def set_state(self, game_state):
        """
        Args:
            game_state: the state to be established in the game
        Returns:
            observation of the game.
        """
        state = (game_state["to_play"], game_state["board_int"])
        return self.engine.set_state(state)

    def expert_action(self):
        return self.engine.expert_action()

    def print(self):
        self.engine.print()
