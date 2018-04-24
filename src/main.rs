extern crate clap;
extern crate rand;

use clap::{App, Arg};
mod cards;
mod game;
mod player;

fn main() {
    let matches = App::new("war")
        .arg(
            Arg::with_name("double")
                .short("d")
                .long("double")
                .help("Use a double standard deck"),
        )
        .get_matches();

    let mut game = game::Game::new(matches.is_present("double"));
    game.play();

    println!(
        "{:?} player won in {} turns and {} wars",
        game.winner(),
        game.turns(),
        game.wars()
    );
}
