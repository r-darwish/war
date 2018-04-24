use cards::{Card, Deck};

pub struct Player {
    playing_deck: Deck,
    reserve_deck: Deck,
}

impl Player {
    pub fn new() -> Self {
        Player {
            playing_deck: Deck::new(),
            reserve_deck: Deck::new(),
        }
    }

    pub fn has_lost(&self) -> bool {
        self.playing_deck.cards().is_empty() && self.reserve_deck.cards().is_empty()
    }

    pub fn number_of_cards(&self) -> usize {
        self.playing_deck.cards().len() + self.reserve_deck.cards().len()
    }

    pub fn draw(&mut self) -> Option<Card> {
        if let Some(card) = self.playing_deck.draw() {
            return Some(card);
        }

        if self.reserve_deck.cards().is_empty() {
            return None;
        }

        while let Some(card) = self.reserve_deck.draw() {
            self.playing_deck.put(card);
        }

        assert!(self.reserve_deck.cards().is_empty());
        assert!(!self.playing_deck.cards().is_empty());
        self.playing_deck.shuffle();

        self.playing_deck.draw()
    }

    pub fn claim(&mut self, card: Card) {
        self.reserve_deck.put(card);
    }
}
