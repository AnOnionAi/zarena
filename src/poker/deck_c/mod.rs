use super::hand_c::card_c::CardC;
use rand::Rng;

#[derive(Debug)]
pub struct DeckC {
    cards: Vec<CardC>,
}

impl DeckC {
    pub fn new() -> DeckC {
        let mut c = Vec::<CardC>::new();
        for i in 2..=14 {
            c.push(CardC::new(i, 0));
            c.push(CardC::new(i, 1));
            c.push(CardC::new(i, 2));
            c.push(CardC::new(i, 3));
        }
        DeckC { cards: c }
    }

    pub fn get_card(&mut self) -> CardC {
        let mut rng = rand::thread_rng();
        if self.cards.len() == 0 {
            return CardC {
                value: 0,
                figure: 4,
            };
        }
        self.cards.remove(rng.gen_range(0..self.cards.len()))
    }
}
