# War
[War](https://en.wikipedia.org/wiki/War_\(card_game\)) is a popular game mostly among
children. Despite playing countless times as a child, I never had the patience to finish a game, so
I wondered how many turns does it actually take to finish one. Luckily, we have computers to answer
this question.

This application simulates a war game between two player and outputs the amount of turns that the
game took, as well as the number of wars that took place. There's also a script for visualizing the
course of the game.

The simulator can play with a standard 54 cards deck, or a double standard deck of 108 cards. This
is due to the fact that a pack of cards might contain two standard decks, in which case kids tend to
mix the two decks and play with 108 cards.

## Usage
Build and run the application using Cargo as you'd build any other Rust application.

## Visualization
The `-o` switch will cause the simulator to output a CSV file recording the course of the game. This
file can be read by the visualization script to generate a graph. It is recommended to run the
script using [fades](https://fades.readthedocs.io/en/release-7-0/) in order to automatically install
its dependencies.

![Graph](doc/example.png?raw=true "Graph")
