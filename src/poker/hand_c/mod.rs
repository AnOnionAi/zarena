pub mod card_c;

use card_c::CardC;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HandC {
    pub cards: Vec<CardC>,
}

impl HandC {
    pub fn new() -> HandC {
        HandC { cards: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn set_card(&mut self, card: CardC) {
        if self.cards.is_empty() {
            self.cards.push(card);
            return;
        }
        for i in 0..self.cards.len() {
            if card.value < self.cards[i].value {
                self.cards.insert(i, card);
                return;
            }
            if i == self.cards.len() - 1 {
                self.cards.push(card);
                return;
            }
        }
    }

    pub fn remove_specific_card(&mut self, card: &CardC) -> CardC {
        for i in 0..self.cards.len() {
            if self.cards[i].value == card.value && self.cards[i].figure == card.figure {
                return self.cards.remove(i);
            }
        }
        CardC::new(0, 4)
    }

    pub fn remove_a_card(&mut self) -> CardC {
        match self.cards.pop() {
            Some(x) => x,
            None => CardC::new(0, 4),
        }
    }

    pub fn clone_hand(&self) -> HandC {
        let mut hand = HandC::new();
        for i in 0..self.cards.len() {
            hand.set_card(self.cards[i].clone_card());
        }
        hand
    }

    #[allow(dead_code)]
    pub fn hand_to_string(&self) -> String {
        let mut s = "".to_string();
        for i in 0..self.cards.len() {
            s += &(" ".to_string() + &self.cards[i].card_to_string());
        }
        s
    }

    pub fn get_value(&self) -> Vec<u8> {
        if self.cards.len() != 5 {
            panic!("hand must be size 5");
        }
        for i in 0..5 {
            if self.cards[i].value == 0 {
                return vec![0];
            }
        }
        let mut pairs_value: HashMap<u8, u8> = HashMap::new();
        for card in self.cards.iter() {
            let counter = pairs_value.entry(card.value).or_insert(0);
            *counter += 1;
        }
        let mut pairs_figure: HashMap<u8, u8> = HashMap::new();
        for card in self.cards.iter() {
            let counter = pairs_figure.entry(card.figure).or_insert(0);
            *counter += 1;
        }
        let mut pairs: Vec<u8> = Vec::new();
        for (key, value) in pairs_value.iter() {
            if *value == 2 {
                pairs.push(key.clone());
            }
        }
        let mut third: Vec<u8> = Vec::new();
        for (key, value) in pairs_value.iter() {
            if *value == 3 {
                third.push(key.clone());
            }
        }
        let mut quartet: Vec<u8> = Vec::new();
        for (key, value) in pairs_value.iter() {
            if *value == 4 {
                quartet.push(key.clone());
            }
        }
        if pairs.len() == 2 {
            // Two pair
            let mut kicker: u8 = 0;
            for card in pairs_value.iter() {
                if *card.1 == 1 {
                    kicker = card.0.clone();
                    break;
                }
            }
            pairs.sort();
            return vec![3, pairs[1], pairs[0], kicker];
        }
        if pairs.len() == 1 && third.len() == 1 {
            // Full house
            return vec![7, third[0], pairs[0]];
        }
        if pairs.len() == 1 {
            // One pair
            let mut kickers: [u8; 3] = [0, 0, 0];
            let mut count = 0;
            for card in pairs_value.iter() {
                if *card.1 == 1 {
                    kickers[count] = card.0.clone();
                    count += 1;
                    if count == 3 {
                        break;
                    }
                }
            }
            kickers.sort();
            return vec![2, pairs[0], kickers[2], kickers[1], kickers[0]];
        }
        if third.len() == 1 {
            // Three of a kind
            let mut kickers: [u8; 2] = [0, 0];
            let mut count = 0;
            for card in pairs_value.iter() {
                if *card.1 == 1 {
                    kickers[count] = card.0.clone();
                    count += 1;
                    if count == 2 {
                        break;
                    }
                }
            }
            kickers.sort();
            return vec![4, third[0], kickers[1], kickers[0]];
        }
        if quartet.len() == 1 {
            // Four of a kind
            let mut kicker: u8 = 0;
            for card in pairs_value.iter() {
                if *card.1 == 1 {
                    kicker = card.0.clone();
                    break;
                }
            }
            return vec![8, quartet[0], kicker];
        }
        let color: bool;
        if pairs_figure.len() == 1 {
            color = true;
        } else {
            color = false;
        }
        let mut consecutive: bool = true;
        let mut x = 0;
        let mut as_value = 14;
        for i in 0..5 {
            if i == 0 {
                if self.cards[0].value == 2 && self.cards[4].value == 14 {
                    as_value = 1;
                }
                x = self.cards[0].value;
            }
            if as_value == 14 {
                if self.cards[i].value != x {
                    consecutive = false;
                    break;
                }
            }
            if as_value == 1 {
                if i == 4 {
                    if self.cards[i].value != 14 {
                        consecutive = false;
                        break;
                    }
                } else {
                    if self.cards[i].value != x {
                        consecutive = false;
                        break;
                    }
                }
            }
            x += 1;
        }
        if color {
            if consecutive {
                if self.cards[4].value == 14 && self.cards[0].value == 10 {
                    // Five of a kind
                    return vec![10, self.cards[4].value];
                }
                // Straight flush
                if as_value == 1 {
                    return vec![9, self.cards[3].value];
                }
                return vec![9, self.cards[4].value];
            } else {
                // Flush
                return vec![
                    6,
                    self.cards[4].value,
                    self.cards[3].value,
                    self.cards[2].value,
                    self.cards[1].value,
                    self.cards[0].value,
                ];
            }
        } else {
            if consecutive {
                // Straight
                if as_value == 1 {
                    return vec![5, self.cards[3].value];
                }
                return vec![5, self.cards[4].value];
            } else {
                // High card
                return vec![
                    1,
                    self.cards[4].value,
                    self.cards[3].value,
                    self.cards[2].value,
                    self.cards[1].value,
                    self.cards[0].value,
                ];
            }
        }
    }

    pub fn hand_to_vec(&self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        for card in self.cards.iter() {
            res.push(card.card_to_int());
        }
        res
    }
}
