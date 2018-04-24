extern crate rand;

mod cards;
mod game;
mod player;

fn main() {
    let mut game = game::Game::new();
    game.play();

    println!(
        "{:?} won in {} turns and {} wars",
        game.winner(),
        game.turns(),
        game.wars()
    );
}
