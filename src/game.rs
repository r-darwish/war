use super::player::Player;
use cards::{Card, Deck};
use enum_map::EnumMap;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::vec::Vec;

#[derive(Debug, EnumMap)]
pub enum Players {
    Blue,
    Red,
}

pub struct Game {
    players: EnumMap<Players, Player>,
    wars: usize,
    turns: usize,
    output_file: Option<BufWriter<File>>,
}

pub struct GameResults {
    pub winner: Players,
    pub wars: usize,
    pub turns: usize,
}

macro_rules! draw {
    ($p:expr) => {
        match $p.draw() {
            Some(card) => card,
            None => {
                return false;
            }
        };
    };
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
            players: enum_map! { _ => Player::new() },
            turns: 0,
            wars: 0,
            output_file: output_file.map(|f| BufWriter::new(File::create(f).unwrap())),
        };

        'cards: loop {
            for player in game.players.values_mut() {
                if let Some(card) = deck.draw() {
                    player.claim(card);
                } else {
                    break 'cards;
                }
            }
        }

        assert!(deck.cards().is_empty());

        game
    }

    fn play_turn(&mut self) -> bool {
        let mut stake = Vec::new();
        let mut war = false;

        loop {
            assert_eq!(self.players.len(), 2);
            let blue_card = draw!(self.players[Players::Blue]);
            let red_card = draw!(self.players[Players::Red]);
            match blue_card.rank().cmp(&red_card.rank()) {
                Ordering::Greater => {
                    self.win(Players::Blue, &mut stake, blue_card, red_card);
                    break;
                }
                Ordering::Less => {
                    self.win(Players::Red, &mut stake, blue_card, red_card);
                    break;
                }
                Ordering::Equal => {
                    war = true;
                    self.wars += 1;

                    stake.push(blue_card);
                    stake.push(red_card);
                    for _ in 0..2 {
                        for player in self.players.values_mut() {
                            stake.push(draw!(player));
                        }
                    }
                }
            }
        }

        self.turns += 1;
        if let Some(ref mut f) = self.output_file {
            f.write(
                format!(
                    "{},{},{}\n",
                    self.players[Players::Red].number_of_cards(),
                    self.players[Players::Blue].number_of_cards(),
                    war
                ).as_bytes(),
            ).unwrap();
        }
        true
    }

    fn win(&mut self, player: Players, stake: &mut Vec<Card>, blue_card: Card, red_card: Card) {
        let player = &mut self.players[player];
        player.claim(blue_card);
        player.claim(red_card);
        for card in stake.drain(..) {
            player.claim(card);
        }
    }

    pub fn play(mut self) -> GameResults {
        if let Some(ref mut f) = self.output_file {
            f.write("red_player_cards,blue_player_cards,war\n".as_bytes())
                .unwrap();
        }

        while self.play_turn() {}

        let winner = self.players
            .iter()
            .find(|&(_, player)| !player.has_lost())
            .unwrap()
            .0;

        GameResults {
            turns: self.turns,
            wars: self.wars,
            winner: winner,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let mut game = Game::new(false, None);
    }
}
