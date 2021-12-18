use super::hand_c::card_c::CardC;
use super::hand_c::HandC;

#[derive(Debug)]
pub struct Player {
    pub id: u8,
    pub credits: u64,
    pub hand: HandC,
    pub hand_value: Vec<u8>,
    pub bet: u64,
    pub total_bet: u64,
    pub in_hand: bool,
    pub in_all_in: bool,
    pub initial_credit: u64,
}

impl Player {
    pub fn new(id: u8, credits: u64) -> Self {
        Player {
            id,
            credits,
            hand: HandC::new(),
            hand_value: vec![],
            bet: 0,
            total_bet: 0,
            in_hand: true,
            in_all_in: false,
            initial_credit: credits,
        }
    }

    pub fn reset(&mut self, pay_back_credit: bool) {
        self.hand = HandC::new();
        self.hand_value = vec![];
        self.bet = 0;
        self.total_bet = 0;
        self.in_hand = true;
        self.in_all_in = false;
        if pay_back_credit {
            self.credits = self.initial_credit;
        }
    }

    #[allow(dead_code)]
    pub fn clone_player(&self) -> Self {
        let mut player = Player::new(self.id, self.credits);
        player.hand = self.hand.clone_hand();
        player.hand_value = self.hand_value.clone();
        player.bet = self.bet;
        player.total_bet = self.total_bet;
        player.in_hand = self.in_hand;
        player.in_all_in = self.in_all_in;
        player
    }

    pub fn set_winner_combination(&mut self, community_hand: &HandC) {
        let n_c = community_hand.len() + 2;
        let mut my_hand = self.hand.clone_hand();
        let mut community_hand = community_hand.clone_hand();
        let mut hand = HandC::new();
        for _i in 0..2 {
            hand.set_card(my_hand.remove_a_card());
        }
        for _i in 2..n_c {
            hand.set_card(community_hand.remove_a_card());
        }
        let mut max_hand = HandC::new();
        for _i in 0..5 {
            max_hand.set_card(CardC::new(0, 4));
        }
        for i in 0..n_c {
            for j in i + 1..n_c {
                let values = [i, j];
                let mut cards: Vec<CardC> = Vec::new();
                for k in 0..n_c - 5 {
                    cards.push(hand.remove_specific_card(&hand.cards[values[k] - k].clone_card()));
                }
                if self.winner(&hand.get_value(), &max_hand.get_value()) == 1 {
                    for _i in 0..5 {
                        max_hand.remove_a_card();
                    }
                    max_hand = hand.clone_hand();
                }
                for card in cards {
                    hand.set_card(card);
                }
            }
        }
        self.hand = max_hand;
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
