use std::io::{BufWriter, Write};
use std::fs::File;
use cards::{Card, Deck};
use std::vec::Vec;
use super::player::Player;

#[derive(Debug)]
pub enum Players {
    Blue,
    Red,
}

pub struct Game {
    blue_player: Player,
    red_player: Player,
    winner: Option<Players>,
    wars: usize,
    turns: usize,
    output_file: Option<BufWriter<File>>,
}

macro_rules! draw {
    ($p:expr) => {
        match $p.draw() {
            Some(card) => card,
            None => {
                return false;
            }
        };
    }
}

impl Game {
    pub fn new(double_deck: bool, output_file: Option<&str>) -> Self {
        let mut deck = Deck::standard();
        if double_deck {
            let mut second = Deck::standard();
            deck.append(&mut second);
        }
        deck.shuffle();

        let mut game = Game {
            blue_player: Player::new(),
            red_player: Player::new(),
            winner: None,
            turns: 0,
            wars: 0,
            output_file: output_file.map(|f| BufWriter::new(File::create(f).unwrap())),
        };

        loop {
            if let Some(card) = deck.draw() {
                game.blue_player.claim(card)
            } else {
                break;
            }

            if let Some(card) = deck.draw() {
                game.red_player.claim(card)
            } else {
                break;
            }
        }

        assert!(deck.cards().is_empty());

        game
    }

    fn play_turn(&mut self) -> bool {
        let mut stake: Vec<Card> = Vec::new();
        let mut war = false;

        loop {
            let blue_card = draw!(self.blue_player);
            let red_card = draw!(self.red_player);

            if blue_card.rank() > red_card.rank() {
                self.blue_player.claim(blue_card);
                self.blue_player.claim(red_card);
                for card in stake.drain(..) {
                    self.blue_player.claim(card);
                }
                break;
            } else if blue_card.rank() < red_card.rank() {
                self.red_player.claim(blue_card);
                self.red_player.claim(red_card);
                for card in stake.drain(..) {
                    self.red_player.claim(card);
                }
                break;
            } else {
                war = true;
                self.wars += 1;

                stake.push(blue_card);
                stake.push(red_card);
                for _ in 0..2 {
                    stake.push(draw!(self.red_player));
                    stake.push(draw!(self.blue_player));
                }
            }
        }

        self.turns += 1;
        if let Some(ref mut f) = self.output_file {
            f.write(
                format!(
                    "{},{},{}\n",
                    self.red_player.number_of_cards(),
                    self.blue_player.number_of_cards(),
                    war
                ).as_bytes(),
            ).unwrap();
        }
        true
    }

    pub fn play(&mut self) {
        if let Some(_) = self.winner {
            panic!("Game already played")
        }

        while self.play_turn() {}

        assert!(self.blue_player.has_lost() || self.red_player.has_lost());

        self.winner = if self.blue_player.has_lost() {
            Some(Players::Red)
        } else {
            Some(Players::Blue)
        };
    }

    pub fn winner(&self) -> &Players {
        self.winner.as_ref().unwrap()
    }

    pub fn turns(&self) -> usize {
        self.turns
    }

    pub fn wars(&self) -> usize {
        self.wars
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let mut game = Game::new();
    }
}
