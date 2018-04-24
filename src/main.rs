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
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("Output file to write the game log"),
        )
        .get_matches();

    let mut game = game::Game::new(matches.is_present("double"), matches.value_of("output"));
    game.play();

    println!(
        "{:?} player won in {} turns and {} wars",
        game.winner(),
        game.turns(),
        game.wars()
    );
}
