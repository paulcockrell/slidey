# Slidey

Slide map puzzle, based on a retro game. Made with Bevy

![Screenshot of game](/assets/game_screen_shot.png "Screen shot of level 1")


## Run

### Web

To run game in a browser during development see [this guide](https://bevy-cheatbook.github.io/platforms/wasm.html)

tldr:

### Setup
```
cargo install wasm-server-runner
```

### Run
```
cargo serve # this is an alias set in `.cargo/config.toml` that runs: `cargo run --target wasm32-unknown-unknown`
```

### Native

```
cargo run
```

## Deploy

For deploying to the web see [this guide](https://bevy-cheatbook.github.io/platforms/wasm/webpage.html)

## How to play

The objective is to collect all of the wizards potion bottles in the dungeon.
Use the arrow keys to move him, he will slide until he hits an object.
If he slides through a bottle, he will pick it up on the way.
Use the spacebar to teleport him to its location, be warned, the teleporter
will switch to the wizards original potition.

Top tip:

The teleporter is a solid object, so use it as a movable bit of wall to help
you navigate the board.


## Assets

Tilemap asset from [kenny.nl](https://www.kenney.nl/assets/tiny-dungeon)
