#[cfg(feature = "python")]
pub mod python;

#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use rand::Rng;

// Types
pub type Deck = Vec<u8>;
pub type Card = u8;
pub type Hand = Vec<Card>;
#[allow(dead_code)]
pub type ObservationVals = [[[u8; 3]; 3]; 3];

// Constants
#[allow(dead_code)]
pub const DECK: [u8; 52] = [
    2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 2, 3,
    4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
];
#[derive(Debug)]
#[allow(dead_code)]
pub struct TwentyOne {
    deck: Deck,
    players_hand: Vec<Hand>,
    players_value: Vec<u8>,
    players_bet: Vec<u64>,
    players_planted: Vec<bool>,
    players_busted: Vec<bool>,
    total_players: u8,
    current_player: u8,
}

impl TwentyOne {
    #[allow(dead_code)]
    pub fn new(n_players: usize) -> TwentyOne {
        let n_players = n_players + 1;
        TwentyOne {
            deck: DECK.to_vec(),
            players_hand: vec![vec![]; n_players],
            players_value: vec![0; n_players],
            players_bet: vec![0; n_players],
            players_planted: vec![false; n_players],
            players_busted: vec![false; n_players],
            total_players: n_players as u8,
            current_player: 1,
        }
    }

    #[allow(dead_code)]
    pub fn get_state(&self) -> (&Vec<Hand>, &Vec<u8>, &Vec<u64>, &Vec<bool>, &Vec<bool>, u8) {
        (
            &self.players_hand,
            &self.players_value,
            &self.players_bet,
            &self.players_planted,
            &self.players_busted,
            self.current_player,
        )
    }

    #[allow(dead_code)]
    pub fn get_total_players(&self) -> u8 {
        self.total_players
    }

    #[allow(dead_code)]
    pub fn step(&mut self, action: u8, change_player: bool) -> (ObservationVals, Vec<i64>, bool) {
        let c_p = self.current_player as usize;
        if action > 12 || !self.legal_actions()[action as usize] {
            panic!("invalid action");
        }
        if self.players_bet[c_p] == 0 {
            self.deal(action);
            return (
                self.get_observation(),
                vec![0; self.total_players as usize],
                self.all_done(),
            );
        }
        if !(self.players_busted[c_p] || self.players_planted[c_p]) {
            if action == 2 {
                self.double_down();
                self.step(1, false);
            } else if action == 0 {
                self.players_planted[c_p] = true;
            } else if action == 1 {
                self.players_hand[c_p] = self.hit(self.players_hand[c_p].clone());
                self.players_value[c_p] = self.deal_card_value(&self.players_hand[c_p]);
            }
            self.players_busted[c_p] = self.is_busted();
        }
        let all_done = self.all_done();
        let reward;
        if all_done {
            reward = self.stand();
        } else {
            reward = vec![0; self.total_players as usize];
        }
        let observation = self.get_observation();
        if change_player {
            loop {
                self.current_player += 1;
                if self.current_player == self.total_players {
                    self.current_player = 1;
                }
                if all_done || !(self.player_done() || self.is_busted()) {
                    break;
                }
            }
        }
        (observation, reward, all_done)
    }

    fn all_done(&self) -> bool {
        let total_players = self.total_players as usize;
        let mut count = 1;
        for player in 1..total_players {
            if (self.players_value[player] > 21 && self.players_value[player] != 100)
                || self.players_planted[player]
                || self.players_busted[player]
            {
                count += 1;
            }
        }
        total_players == count
    }

    fn stand(&mut self) -> Vec<i64> {
        self.dealer_plays();
        let mut res: Vec<i64> = Vec::new();
        for player in 0..self.total_players as usize {
            match self.get_reward(player) {
                2 => {
                    let reward = 3 * self.players_bet[player];
                    res.push((reward / 3 * 2) as i64);
                }
                1 => {
                    let reward = 2 * self.players_bet[player];
                    res.push((reward / 2) as i64);
                }
                0 => {
                    res.push(0);
                }
                -1 => {
                    let reward = self.players_bet[player];
                    res.push(-(reward as i64));
                }
                _ => {}
            }
        }
        res
    }

    fn get_observation(&self) -> ObservationVals {
        let mut res: ObservationVals = [[[0; 3]; 3]; 3];
        let vals: [u8; 3] = [self.players_value[1], self.players_hand[0][0], 0];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    res[i][j][k] = vals[i];
                }
            }
        }
        res
    }

    pub fn legal_actions(&self) -> [bool; 12] {
        // <----bet---->
        // the first step is to bet
        if self.players_bet[self.current_player as usize] == 0 {
            return [
                false, false, false, false, true, true, true, true, true, true, true, true,
            ];
        }
        // <----plays---->
        // 0 = stand
        // 1 = HIT
        // 2 = double down
        // 3 = pull apart (currently disabled)
        let c_p = self.current_player as usize;
        if self.players_hand[c_p].len() == 2 {
            return [
                true, true, true, false, false, false, false, false, false, false, false, false,
            ];
        }
        if self.players_busted[c_p] || self.players_planted[c_p] {
            return [
                false, false, false, false, false, false, false, false, false, false, false, false,
            ];
        }
        [
            true, true, false, false, false, false, false, false, false, false, false, false,
        ]
    }

    fn double_down(&mut self) {
        let c_p = self.current_player as usize;
        self.players_bet[c_p] *= 2;
    }

    // won->1 dealer_wins->-1 draw->0 wins2:3->2
    fn get_reward(&self, player: usize) -> i8 {
        let player = player;
        if self.players_value[player] == 100 && self.players_value[0] != 100 {
            return 2;
        }
        if self.players_value[player] == 100 && self.players_value[0] == 100 {
            return 0;
        }
        if self.players_value[player] <= 21 && self.players_value[0] < self.players_value[player] {
            return 1;
        }
        if self.players_value[player] <= 21
            && (self.players_value[0] > 21 && self.players_value[0] != 100)
        {
            return 1;
        }
        if self.players_value[player] > 21
            && (self.players_value[0] > 21 && self.players_value[0] != 100)
        {
            return 0;
        }
        if self.players_value[player] > 21 {
            return -1;
        }
        if self.players_value[player] == self.players_value[0] {
            return 0;
        }
        return -1;
    }

    // Bet 0->$1, 1->$5, 2->$10, 3->$25, 4->$50, 5->$100, 6->$500, 7->$1000
    fn deal(&mut self, bet: u8) -> bool {
        if bet > 11 {
            panic!("invalid action");
        }
        let c_p = self.current_player as usize;
        let bets = [1, 5, 10, 25, 50, 100, 500, 1000];
        let playerbet = bets[bet as usize - 4];
        self.players_bet[c_p] = playerbet;
        if self.current_player == 1 {
            for _i in 0..2 {
                self.players_hand[0] = self.hit(self.players_hand[0].clone());
                self.players_value[0] = self.deal_card_value(&self.players_hand[0]);
            }
        }
        self.step(1, false);
        self.step(1, true);
        true
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) -> ObservationVals {
        let t_p = self.total_players as usize;
        let observation = [[[0; 3]; 3]; 3];
        self.deck = DECK.to_vec();
        self.players_hand = vec![vec![]; t_p];
        self.players_value = vec![0; t_p];
        self.players_bet = vec![0; t_p];
        self.players_planted = vec![false; t_p];
        self.players_busted = vec![false; t_p];
        self.current_player = 1;
        observation
    }

    #[allow(dead_code)]
    pub fn to_play(&self) -> u8 {
        self.current_player
    }

    pub fn player_done(&self) -> bool {
        let c_p = self.current_player as usize;
        self.players_busted[c_p] || self.players_planted[c_p]
    }

    fn hit(&mut self, mut hand: Hand) -> Hand {
        let mut rng = rand::thread_rng();
        if self.deck.len() == 0 {
            self.deck = DECK.to_vec();
        }
        let card = self.deck.remove(rng.gen_range(0..self.deck.len()));
        hand.push(card);
        hand
    }

    // come back 100 when it's blackjack
    fn deal_card_value(&self, hand: &Hand) -> u8 {
        let mut value = 0;
        for card in hand.iter() {
            if *card == 11 || *card == 12 || *card == 13 {
                value += 10;
            } else if *card == 14 {
                if value >= 11 {
                    value += 1;
                } else {
                    value += 11;
                }
            } else {
                value += card;
            }
        }
        if value == 21 && hand.len() == 2 {
            return 100;
        }
        value
    }

    fn dealer_plays(&mut self) {
        let players_still_alive = !self
            .players_value
            .iter()
            .filter(|x| x <= &&21 || x == &&100)
            .collect::<Vec<&u8>>()
            .is_empty();
        if players_still_alive {
            while self.players_value[0] <= 16 {
                self.players_hand[0] = self.hit(self.players_hand[0].clone());
                self.players_value[0] = self.deal_card_value(&self.players_hand[0]);
            }
        }
    }

    fn is_busted(&self) -> bool {
        let c_p = self.current_player as usize;
        if self.players_value[c_p] > 21 && self.players_value[c_p] != 100 {
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn render(&self) {
        for player in 1..self.total_players as usize {
            println!(
                "Player {} Hand: {:?} Player value: {}",
                player, self.players_hand[player], self.players_value[player]
            );
        }
        println!(
            "Dealer Hand: {:?} Dealer value: {}",
            self.players_hand[0], self.players_value[0]
        );
    }
}
