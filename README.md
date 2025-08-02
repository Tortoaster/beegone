# Beegone

A chess-like game themed around bees, featuring:

- A Rust backend
- A svelte frontend
- 7 types of pieces with different abilities
- Frontend move calculation by compiling the game's logic to WASM
- A computer player that plays pretty okay, using [rival](https://github.com/Tortoaster/rival)
- No win conditions implemented (I got distracted)

Visit [this page](https://beegone.tortoaster.com/) to try it!

## Rules

The game is played by two players, taking turns. Each player starts with a queen, a drone, and two nurses. During your
turn, you can either move a piece, capture an enemy piece or use a piece's special ability.

### Moving

Every piece can move to any of the six tiles adjacent to it, as long as there is no other piece or wall there. Moreover,
if a piece is adjacent to a piece of the same team, it can 'hop over' that piece, as long as that tile is accessible (
either empty or they can capture the piece standing there).

### Capturing

Most pieces can capture an enemy piece adjacent to them by moving their piece onto the same tile. However:

* Guards can only capture other guards, gatherers, builders, nurses, workers and drones
* Gatherers can only capture other gatherers, builders, nurses, workers and drones
* Builders can only capture other builders, nurses, workers and drones
* Nurses can only capture other nurses, workers and drones
* Workers can only capture other workers and drones
* Drones can only capture other drones

Summarized, pieces can only capture the same or 'weaker' pieces, where guards > gatherers > builders > nurses >
workers > drones. Additionally:

* Queens are unable to capture at all, and cannot be captured normally
* Walls can only be captured by builders

### Special abilities

Some pieces have special abilities:

* Queens can spawn drones on any adjacent tile, or, if a drone is already adjacent, she can spawn workers on any
  adjacent tile
* Nurses can promote adjacent workers to builders, gatherers or guards
* Gatherers can walk in a straight line any number of tiles, as long as there is nothing blocking the way (they may also
  do this right after hopping over an ally)
* Builders can place walls on adjacent tiles

### Winning the game

As I said, I never implemented a win condition, so you can continue playing for as long as you want. Originally, I
intended to implement the following win conditions:

* It's your opponent's turn, but they're unable to make a move
* You capture both your opponent's nurses
* You have surrounded your opponent's queen with 3 guards

But the second didn't feel fair if you already had many promoted pieces to win the game with, and the third made the
game drag on for way too long. Instead, you can pretend that guards can capture enemy queens normally, and doing so ends
the game.

To further limit how long the game takes, you can pretend that a queen may only spawn up to 3 drones before getting
tired, and nurses may only promote 15 pieces in total. Stalemates may happen, I didn't playtest enough.

## Local Development

### Install dependencies

```shell
cargo install --locked just typeshare-cli wasm-bindgen-cli
rustup target add wasm32-unknown-unknown
```

### Build WASM

Running `just build` will compile the `beegone` crate to WASM and use it to recreate the `beegone-js` library. Ideally,
this would be done automatically in the frontend's `prepare` step, but CloudFlare Pages' deployment process cannot run
this command (that's also why the `beegone-js/src` folder isn't ignored), so it's a manual step for now.

### Run project

Run `just run` (or just `just`) to run the project.

## License

Beegone\
Copyright (C) 2024 Rick van der Wal

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
