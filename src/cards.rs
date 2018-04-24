use rand::{thread_rng, Rng};
use std::vec::Vec;

#[derive(Debug)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spade,
}

#[derive(Debug)]
pub enum SuitRank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}

impl<'a> From<&'a SuitRank> for Rank {
    fn from(suit_rank: &SuitRank) -> Self {
        match suit_rank {
            &SuitRank::Two => Rank::Two,
            &SuitRank::Three => Rank::Three,
            &SuitRank::Four => Rank::Four,
            &SuitRank::Five => Rank::Five,
            &SuitRank::Six => Rank::Six,
            &SuitRank::Seven => Rank::Seven,
            &SuitRank::Eight => Rank::Eight,
            &SuitRank::Nine => Rank::Nine,
            &SuitRank::Ten => Rank::Ten,
            &SuitRank::Jack => Rank::Jack,
            &SuitRank::Queen => Rank::Queen,
            &SuitRank::King => Rank::King,
            &SuitRank::Ace => Rank::Ace,
        }
    }
}

#[derive(Debug)]
pub enum Color {
    Black,
    Red,
}

#[derive(Debug)]
pub enum Card {
    SuitCard(Suit, SuitRank),
    Joker(Color),
}

impl Card {
    pub fn rank(&self) -> Rank {
        match self {
            &Card::Joker(_) => Rank::Joker,
            &Card::SuitCard(_, ref rank) => Rank::from(rank),
        }
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn standard() -> Self {
        Self {
            cards: vec![
                Card::SuitCard(Suit::Hearts, SuitRank::Two),
                Card::SuitCard(Suit::Hearts, SuitRank::Three),
                Card::SuitCard(Suit::Hearts, SuitRank::Four),
                Card::SuitCard(Suit::Hearts, SuitRank::Five),
                Card::SuitCard(Suit::Hearts, SuitRank::Six),
                Card::SuitCard(Suit::Hearts, SuitRank::Seven),
                Card::SuitCard(Suit::Hearts, SuitRank::Eight),
                Card::SuitCard(Suit::Hearts, SuitRank::Nine),
                Card::SuitCard(Suit::Hearts, SuitRank::Ten),
                Card::SuitCard(Suit::Hearts, SuitRank::Jack),
                Card::SuitCard(Suit::Hearts, SuitRank::Queen),
                Card::SuitCard(Suit::Hearts, SuitRank::King),
                Card::SuitCard(Suit::Hearts, SuitRank::Ace),
                Card::SuitCard(Suit::Diamonds, SuitRank::Two),
                Card::SuitCard(Suit::Diamonds, SuitRank::Three),
                Card::SuitCard(Suit::Diamonds, SuitRank::Four),
                Card::SuitCard(Suit::Diamonds, SuitRank::Five),
                Card::SuitCard(Suit::Diamonds, SuitRank::Six),
                Card::SuitCard(Suit::Diamonds, SuitRank::Seven),
                Card::SuitCard(Suit::Diamonds, SuitRank::Eight),
                Card::SuitCard(Suit::Diamonds, SuitRank::Nine),
                Card::SuitCard(Suit::Diamonds, SuitRank::Ten),
                Card::SuitCard(Suit::Diamonds, SuitRank::Jack),
                Card::SuitCard(Suit::Diamonds, SuitRank::Queen),
                Card::SuitCard(Suit::Diamonds, SuitRank::King),
                Card::SuitCard(Suit::Diamonds, SuitRank::Ace),
                Card::SuitCard(Suit::Clubs, SuitRank::Two),
                Card::SuitCard(Suit::Clubs, SuitRank::Three),
                Card::SuitCard(Suit::Clubs, SuitRank::Four),
                Card::SuitCard(Suit::Clubs, SuitRank::Five),
                Card::SuitCard(Suit::Clubs, SuitRank::Six),
                Card::SuitCard(Suit::Clubs, SuitRank::Seven),
                Card::SuitCard(Suit::Clubs, SuitRank::Eight),
                Card::SuitCard(Suit::Clubs, SuitRank::Nine),
                Card::SuitCard(Suit::Clubs, SuitRank::Ten),
                Card::SuitCard(Suit::Clubs, SuitRank::Jack),
                Card::SuitCard(Suit::Clubs, SuitRank::Queen),
                Card::SuitCard(Suit::Clubs, SuitRank::King),
                Card::SuitCard(Suit::Clubs, SuitRank::Ace),
                Card::SuitCard(Suit::Spade, SuitRank::Two),
                Card::SuitCard(Suit::Spade, SuitRank::Three),
                Card::SuitCard(Suit::Spade, SuitRank::Four),
                Card::SuitCard(Suit::Spade, SuitRank::Five),
                Card::SuitCard(Suit::Spade, SuitRank::Six),
                Card::SuitCard(Suit::Spade, SuitRank::Seven),
                Card::SuitCard(Suit::Spade, SuitRank::Eight),
                Card::SuitCard(Suit::Spade, SuitRank::Nine),
                Card::SuitCard(Suit::Spade, SuitRank::Ten),
                Card::SuitCard(Suit::Spade, SuitRank::Jack),
                Card::SuitCard(Suit::Spade, SuitRank::Queen),
                Card::SuitCard(Suit::Spade, SuitRank::King),
                Card::SuitCard(Suit::Spade, SuitRank::Ace),
                Card::Joker(Color::Black),
                Card::Joker(Color::Red),
            ].into_iter()
                .collect(),
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        rng.shuffle(&mut self.cards)
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn put(&mut self, card: Card) {
        self.cards.push(card)
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn append(&mut self, other: &mut Self) {
        self.cards.append(&mut other.cards);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks() {
        assert_eq!(
            Card::Joker(Color::Black).rank(),
            Card::Joker(Color::Red).rank()
        );

        assert!(
            Card::Joker(Color::Black).rank() > Card::SuitCard(Suit::Spade, SuitRank::Ace).rank()
        );
    }
}
