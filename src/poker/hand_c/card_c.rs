#[derive(Debug)]
pub struct CardC {
    pub value: u8,
    pub figure: u8,
}

impl CardC {
    pub fn new(v: u8, f: u8) -> CardC {
        CardC {
            value: v,
            figure: f,
        }
    }

    pub fn clone_card(&self) -> CardC {
        CardC::new(self.value, self.figure)
    }

    pub fn card_to_int(&self) -> u8 {
        return self.figure * 15 + self.value;
    }

    #[allow(dead_code)]
    pub fn card_to_string(&self) -> String {
        let v = match self.value {
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            14 => "A".to_string(),
            _ => self.value.to_string(),
        };
        match self.figure {
            0 => v + "-♥",
            1 => v + "-♠",
            2 => v + "-♣",
            3 => v + "-♦",
            _ => "was over".to_string(),
        }
    }
}
