import gym
from zarena.gym_poker import PokerEngine

# import random
# from tqdm import tqdm

# POKER GYM ENVIRONMENT CLASS
# ---------------------------
class PokerEnv(gym.Env):
    """
    Game wrapper.
    """

    def __init__(self, n_players=2, infinite_game=True):
        # engine
        self.engine = PokerEngine(n_players, infinite_game)

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
        state_a, state_b = self.engine.get_state_a(), self.engine.get_state_b()
        community_cards = []
        if len(state_a[0]) > 0:
            community_cards = self.hand_to_string(state_a[0])
        players_hand = []
        for player in state_a[1]:
            players_hand.append(self.hand_to_string(player["hand"]["cards"]))
        lista_a = list(state_a)
        lista_a[0] = community_cards
        for player in lista_a[1]:
            player["hand"] = players_hand[player["id"]]
        state_a = tuple(lista_a)
        state = state_a + state_b
        return state

    def get_total_players(self):
        """
        Returns:
            Integer total players + dealer
        """
        return self.engine.get_total_players()

    def card_to_string(self, card):
        figure = card // 15
        value = card % 15
        if value == 11:
            v_str = "J"
        elif value == 12:
            v_str = "Q"
        elif value == 13:
            v_str = "K"
        elif value == 14:
            v_str = "A"
        else:
            v_str = str(value)

        if figure == 0:
            f_str = "-♥"
        elif figure == 1:
            f_str = "-♠"
        elif figure == 2:
            f_str = "-♣"
        elif figure == 3:
            f_str = "-♦"
        else:
            f_str = "was over"

        return v_str + f_str

    def hand_to_string(self, hand):
        res = []
        for card in hand:
            res.append(self.card_to_string(card))
        return res
