# Minesweeper (in Rust tho)

Playable at: [nikhil-42.github.io/minesweeper](https://nikhil-42.github.io/minesweeper)

The classic Minesweeper game implemented in Rust using the macroquad library.

Mainly implemented because I'm jet-lagged and everyone is asleep. Also, I dislike the 
Google Minesweeper controls. 

## Features
- Spawn protection to prevent unsolvable starts
- Customizable grid size and mine count
- Click to reveal tiles
- Click revealed tile to reveal unflagged neighbors (recursive)
- Right-click to flag tiles
- Timer to track game duration
- Simple graphics with textures for flags, mines, and numbers

## Bugs
- The RNG isn't very random, so the same mine positions can appear on different runs.
- The game doesn't handle resizing the window well.
- No fancy animations or effects, just basic tile revealing.
- No sound effects or music (yet?).
