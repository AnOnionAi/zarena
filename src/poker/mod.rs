mod deck_c;
mod hand_c;
mod player;
#[cfg(feature = "python")]
pub mod python;

use deck_c::DeckC;
use hand_c::HandC;
use player::Player;

#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use std::{u64, usize, vec};

// Types
#[allow(dead_code)]
pub type ObservationVals = [[[u64; 5]; 5]; 2];

#[derive(Debug)]
#[allow(dead_code)]
pub struct Poker {
    deck: DeckC,
    players: Vec<Player>,
    n_players_in_hand: u8,
    community_cards: HandC,
    total_players: u8,
    current_player: u8,
    turn_in_phase: u8,
    poker_phase: u8,
    bet_phase: u64,
    button: u8,
    been_all_in: bool,
    hole: Vec<u64>,
    hole_limbs: Vec<Vec<u8>>,
    infinite_credits: bool,
}

impl Poker {
    #[allow(dead_code)]
    pub fn new(p_credits: Vec<u64>, infinite_credits: bool) -> Self {
        let n_players = p_credits.len();
        let mut players: Vec<Player> = Vec::new();
        for i in 0..n_players {
            players.push(Player::new(i as u8, p_credits[i]));
        }
        Poker {
            deck: DeckC::new(),
            players,
            n_players_in_hand: n_players as u8,
            community_cards: HandC::new(),
            total_players: n_players as u8,
            current_player: 1,
            turn_in_phase: 0,
            poker_phase: 0,
            bet_phase: 0,
            button: 0,
            been_all_in: false,
            hole: vec![0],
            hole_limbs: Vec::new(),
            infinite_credits,
        }
    }

    #[allow(dead_code)]
    pub fn get_state(
        &self,
    ) -> (
        Vec<u8>,
        &Vec<Player>,
        &Vec<u64>,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u64,
    ) {
        (
            self.community_cards.hand_to_vec(),
            &self.players,
            &self.hole,
            self.total_players,
            self.n_players_in_hand,
            self.current_player,
            self.button,
            self.poker_phase,
            self.turn_in_phase,
            self.bet_phase,
        )
    }

    #[allow(dead_code)]
    pub fn get_total_players(&self) -> u8 {
        self.total_players
    }

    #[allow(dead_code)]
    pub fn step(&mut self, action: u8, change_player: bool) -> (ObservationVals, Vec<i64>, bool) {
        let c_p = self.current_player as usize;
        let mut reward: Vec<i64> = vec![0; self.total_players as usize];
        if action > 11 || !self.legal_actions().contains(&action) {
            panic!("invalid action");
        }
        if action == 0 {
            self.to_bet(5);
        }
        if action == 1 {
            self.to_bet(10);
        }
        if action == 2 {
            self.players[c_p].in_hand = false;
            self.n_players_in_hand -= 1;
            if self.n_players_in_hand == 1 {
                self.poker_phase = 3;
            }
        }
        if action == 4 {
            self.to_bet(10);
        }
        if action == 5 {
            self.to_bet(self.bet_phase - self.players[c_p].bet);
        }
        if action == 6 {
            self.to_bet(25);
        }
        if action == 7 {
            self.to_bet(50);
        }
        if action == 8 {
            self.to_bet(100);
        }
        if action == 9 {
            self.to_bet(500);
        }
        if action == 10 {
            self.to_bet(1000);
        }
        if action == 11 {
            self.all_in();
        }
        self.turn_in_phase += 1;
        let is_turn_completed = self.is_turn_completed();
        if is_turn_completed {
            if self.been_all_in {
                for _i in 0..5 - self.community_cards.len() {
                    self.community_cards.set_card(self.deck.get_card());
                }
                self.poker_phase = 3;
            }
            self.poker_phase += 1;
            if self.poker_phase == 1 {
                for _i in 0..3 {
                    self.community_cards.set_card(self.deck.get_card());
                }
            } else if self.poker_phase == 4 {
                self.collect_bets();
                if self.n_players_in_hand > 1 {
                    self.assign_winning_combination();
                }
                reward = self.get_reward();
                self.next_dealer();
                self.reset();
                return (self.get_observation(), reward, self.is_done());
            } else {
                self.community_cards.set_card(self.deck.get_card());
            }
            self.turn_in_phase = 0;
            self.current_player = self.button;
            self.collect_bets();
            self.bet_phase = 0;
        }
        if change_player {
            self.next_player();
        }
        (self.get_observation(), reward, false)
    }

    fn is_done(&self) -> bool {
        self.button == 0
    }

    fn all_in(&mut self) {
        let c_p = self.current_player as usize;
        self.players[c_p].in_all_in = true;
        self.to_bet(self.players[c_p].credits);
        self.been_all_in = true;
    }

    fn assign_winning_combination(&mut self) {
        for player in self.players.iter_mut() {
            if player.in_hand {
                player.set_winner_combination(&self.community_cards);
                player.hand_value = player.hand.get_value();
            }
        }
    }

    fn get_reward(&mut self) -> Vec<i64> {
        let mut reward: Vec<i64> = vec![0; self.total_players as usize];
        for i in 0..self.hole.len() {
            let mut winner = vec![0];
            let mut w_players = vec![];
            if self.n_players_in_hand > 1 {
                for player in self.players.iter() {
                    if player.in_hand && self.hole_limbs[i].contains(&player.id) {
                        let w = self.winner(&winner, &player.hand_value);
                        if w == 2 {
                            winner = player.hand_value.clone();
                            w_players = vec![];
                            w_players.push(player.id);
                        } else if w == 0 {
                            w_players.push(player.id);
                        }
                    }
                }
            } else {
                w_players.push(
                    self.players
                        .iter()
                        .filter(|p| p.in_hand)
                        .collect::<Vec<&Player>>()[0]
                        .id,
                );
            }
            let profit = self.hole[i] / w_players.len() as u64;
            for player in self.players.iter() {
                if player.in_hand {
                    if w_players.contains(&player.id) {
                        reward[player.id as usize] += profit as i64;
                    } else {
                        reward[player.id as usize] = -(player.total_bet as i64);
                    }
                } else {
                    reward[player.id as usize] = -(player.total_bet as i64);
                }
            }
        }
        self.return_ernings(&reward);
        reward
    }

    fn return_ernings(&mut self, reward: &Vec<i64>) {
        for player in self.players.iter_mut() {
            if reward[player.id as usize] > 0 {
                player.credits += reward[player.id as usize] as u64;
            }
        }
    }

    fn next_dealer(&mut self) {
        self.button += 1;
        self.button = self.button % self.total_players;
    }

    fn next_player(&mut self) {
        let c_p = &mut self.current_player;
        loop {
            *c_p += 1;
            *c_p %= self.players.len() as u8;
            if self.players[*c_p as usize].in_hand {
                break;
            }
        }
    }

    fn collect_bets(&mut self) {
        if self.been_all_in {
            self.players.sort_by_key(|x| x.bet);
            let mut bet_levels: Vec<u64> = vec![];
            for player in self.players.iter() {
                if player.in_hand {
                    if bet_levels.len() == 0 {
                        bet_levels.push(player.bet);
                    } else if player.bet != bet_levels[bet_levels.len() - 1] {
                        bet_levels.push(player.bet);
                    }
                }
            }
            for _i in 1..bet_levels.len() - 1 {
                self.hole.push(0);
            }
            if bet_levels.len() == 1 {
                self.hole_limbs = vec![vec![]];
            } else {
                self.hole_limbs = vec![vec![]; bet_levels.len() - 1];
            }
            for i in 0..bet_levels.len() {
                for player in self.players.iter_mut() {
                    if player.bet >= bet_levels[i] {
                        if i < self.hole.len() {
                            self.hole[i] += bet_levels[i];
                            player.bet -= bet_levels[i];
                            self.hole_limbs[i].push(player.id);
                        } else {
                            player.total_bet -= bet_levels[bet_levels.len() - 1];
                            player.credits = bet_levels[bet_levels.len() - 1];
                        }
                    } else {
                        self.hole[0] += player.bet;
                        player.bet = 0;
                    }
                }
                let temp = bet_levels.clone();
                for k in 0..bet_levels.len() {
                    if bet_levels[k] > 0 {
                        bet_levels[k] -= temp[i];
                    }
                }
            }
            self.players.sort_by_key(|x| x.id);
        } else {
            self.hole_limbs = vec![vec![]];
            for player in self.players.iter_mut() {
                self.hole[0] += player.bet;
                player.bet = 0;
                self.hole_limbs[0].push(player.id);
            }
        }
    }

    fn to_bet(&mut self, bet: u64) {
        let c_p = self.current_player as usize;
        if bet > self.players[c_p].credits {
            panic!("the bet cannot be greater than the credit");
        }
        self.players[c_p].bet += bet;
        self.players[c_p].credits -= bet;
        self.players[c_p].total_bet += bet;
        if self.players[c_p].bet > self.bet_phase {
            self.bet_phase = self.players[c_p].bet;
        }
    }

    fn is_turn_completed(&self) -> bool {
        let t_p = self.total_players as usize;
        let mut count = 0;
        for player in self.players.iter() {
            if self.bet_phase == player.bet || player.in_all_in || !player.in_hand {
                count += 1;
            }
        }
        if self.poker_phase == 0 && !self.been_all_in {
            t_p == count && self.turn_in_phase >= self.total_players + 2
        } else {
            t_p == count && self.turn_in_phase >= self.n_players_in_hand as u8
        }
    }

    fn get_observation(&self) -> ObservationVals {
        let mut res: ObservationVals = [[[0; 5]; 5]; 2];
        let c_p = self.current_player as usize;
        for i in 0..self.community_cards.len() {
            res[0][0][i] = self.community_cards.cards[i].card_to_int() as u64;
        }
        for i in 0..self.players[c_p].hand.len() {
            res[0][1][i] = self.players[c_p].hand.cards[i].card_to_int() as u64;
        }
        for i in 0..self.players.len() {
            let x = i % 5;
            let y = i / 5;
            res[1][y][x] = self.players[i].credits;
        }
        res
    }

    pub fn legal_actions(&self) -> Vec<u8> {
        // <----plays---->
        // 0.- small blind
        // 1.- big blind
        // 2.- fold
        // 3.- check
        // 4.- bet
        // 5.- call
        // 6.- raise to 25
        // 7.- raise to 50
        // 8.- raise to 100
        // 9.- raise to 500
        // 10.- raise to 1000
        // 11.- all in
        let c_p = self.current_player as usize;
        let mut actions = Vec::new();
        if self.poker_phase == 0 && self.players[c_p].bet == 0 {
            if c_p as u8 == (self.button + 1) % self.total_players {
                return vec![0];
            }
            if c_p as u8 == (self.button + 2) % self.total_players {
                return vec![1];
            }
        }
        if self.bet_phase == 0 {
            actions.push(3);
            actions.push(4);
        }
        if self.bet_phase == self.players[c_p].bet && self.bet_phase != 0 {
            actions.push(3);
            actions.push(6);
        }
        if self.players[c_p].bet < self.bet_phase {
            let bets: [u64; 5] = [25, 50, 100, 500, 1000];
            let n_action: [u8; 5] = [6, 7, 8, 9, 10];
            actions.push(2);
            if !self.been_all_in {
                if self.players[c_p].credits >= self.bet_phase {
                    actions.push(5);
                }
                for i in 0..5 {
                    if self.bet_phase < bets[i] && bets[i] < self.players[c_p].credits {
                        actions.push(n_action[i]);
                    }
                }
                actions.push(11);
            } else {
                actions.push(11);
            }
        }
        actions
    }

    pub fn reset(&mut self) -> ObservationVals {
        let mut hands: Vec<HandC> = Vec::new();
        for _ in 0..self.total_players {
            hands.push(HandC::new());
        }
        let observation = self.get_observation();
        self.deck = DeckC::new();
        self.community_cards = HandC::new();
        self.turn_in_phase = 0;
        self.poker_phase = 0;
        self.bet_phase = 0;
        self.button = 0;
        self.been_all_in = false;
        self.hole = vec![0];
        let pay_back_credit = self.infinite_credits;
        self.players
            .iter_mut()
            .for_each(|x| x.reset(pay_back_credit));
        self.current_player = 1;
        self.n_players_in_hand = self.total_players;
        for player in self.players.iter_mut() {
            player.hand.set_card(self.deck.get_card());
            player.hand.set_card(self.deck.get_card());
        }
        self.hole_limbs = Vec::new();
        observation
    }

    #[allow(dead_code)]
    pub fn to_play(&self) -> u8 {
        self.current_player
    }

    #[allow(dead_code)]
    fn render(&self) {
        println!("Phase {}", self.poker_phase);
        for i in 0..self.hole.len() {
            if i == 0 {
                print!("Main pot {} ", self.hole[i]);
            } else {
                print!(" Side pot {}", self.hole[i]);
            }
        }
        println!("Bet {}", self.bet_phase);
        if !self.community_cards.cards.is_empty() {
            println!("Community cards {}", self.community_cards.hand_to_string());
        }
        for player in self.players.iter().filter(|x| x.in_hand) {
            print!(
                "Player {} Hand: {} Credits: ${:?},",
                player.id,
                player.hand.hand_to_string(),
                player.credits
            );
            print!(" Total bet {},", player.total_bet);
            if self.button == player.id {
                print!(" ⨀ ");
            }
            print!(" Bet {}", player.bet);
            println!();
        }
    }

    fn winner(&self, value_a: &Vec<u8>, value_b: &Vec<u8>) -> u8 {
        for i in 0..value_a.len() {
            if value_a[i] > value_b[i] {
                return 1;
            }
            if value_b[i] > value_a[i] {
                return 2;
            }
        }
        return 0;
    }
}
