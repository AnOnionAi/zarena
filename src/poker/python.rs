// The python library
// use pyo3::{exceptions::PyException, types::{PyTuple, PyDict}};
use pyo3::{prelude::*, types::PyDict};

use super::{HandC, Player, Poker};

// PYTHON MODULE
// ---------------------------------------------------------
// ---------------------------------------------------------
#[pyclass]
pub struct PokerEngine {
    game: Poker,
}

#[pymethods]
impl PokerEngine {
    #[new]
    fn new(n_players: usize, infinite_credits: bool) -> Self {
        PokerEngine {
            game: Poker::new(vec![100000; n_players], infinite_credits),
        }
    }

    pub fn legal_actions(&self) -> PyResult<Vec<u8>> {
        // <----plays---->
        // 0.- big blind
        // 1.- small blind
        // 2.- fold
        // 3.- check
        // 4.- bet
        // 5.- call
        // 6.- raise to 25
        // 7.- raise to 50
        // 8.- raise to 100
        // 9.- raise to 500
        // 10.- raise to 1000
        let py_legal_actions = self.game.legal_actions();
        Ok(py_legal_actions)
    }

    pub fn step(&mut self, action: u8) -> PyResult<(Vec<Vec<Vec<u64>>>, i64, bool)> {
        // a = observation
        // b = reward
        // c = done
        let (_a, b, c) = self.game.step(action, true);
        let a = array_to_vector(_a);
        Ok((a, b[1], c))
    }

    pub fn get_state_a<'a>(
        &self,
        _py: Python<'a>,
    ) -> PyResult<(Vec<u8>, Vec<&'a PyDict>, Vec<u64>)> {
        let (community_cards, players, pots, _, _, _, _, _, _, _) = self.game.get_state();
        let mut players_py = Vec::new();
        for player in players.iter() {
            let player_py = PyDict::new(_py);
            player.to_py_object(_py, player_py);
            players_py.push(player_py);
        }
        Ok((community_cards.clone(), players_py, pots.clone()))
    }

    pub fn get_state_b(&self) -> PyResult<(u8, u8, u8, u8, u8, u8, u64)> {
        let (
            _,
            _,
            _,
            total_players,
            n_players_in_hand,
            current_player,
            button,
            poker_phase,
            turn_in_phase,
            bet_phase,
        ) = self.game.get_state();
        Ok((
            total_players,
            n_players_in_hand,
            current_player,
            button,
            poker_phase,
            turn_in_phase,
            bet_phase,
        ))
    }

    pub fn get_total_players(&self) -> PyResult<u8> {
        Ok(self.game.get_total_players())
    }

    pub fn to_play(&self) -> PyResult<u8> {
        Ok(self.game.to_play())
    }

    pub fn reset(&mut self) -> PyResult<Vec<Vec<Vec<u64>>>> {
        let res = array_to_vector(self.game.reset());
        Ok(res)
    }
}

// conversion functions
pub fn array_to_vector(_a: [[[u64; 5]; 5]; 2]) -> Vec<Vec<Vec<u64>>> {
    let mut a = Vec::new();
    for i in 0..2 {
        let mut y = Vec::new();
        for j in 0..5 {
            let mut x = Vec::new();
            for k in 0..5 {
                x.push(_a[i][j][k]);
            }
            y.push(x);
        }
        a.push(y);
    }
    a
}

impl Player {
    pub fn to_py_object(&self, _py: Python, dict: &PyDict) {
        dict.set_item("id", self.id).unwrap();
        dict.set_item("credits", self.credits).unwrap();
        let hand_py = PyDict::new(_py);
        self.hand.to_py_object(hand_py);
        dict.set_item("hand", hand_py).unwrap();
        dict.set_item("hand_value", &self.hand_value).unwrap();
        dict.set_item("bet", self.bet).unwrap();
        dict.set_item("total_bet", self.total_bet).unwrap();
        dict.set_item("in_hand", self.in_hand).unwrap();
        dict.set_item("in_all_in", self.in_all_in).unwrap();
    }
}

impl HandC {
    pub fn to_py_object(&self, dict: &PyDict) {
        let mut cards = Vec::new();
        for card in self.cards.iter() {
            cards.push(card.card_to_int());
        }
        dict.set_item("cards", cards).unwrap();
    }
}
