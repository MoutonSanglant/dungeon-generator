# Rust-like Dungeon Generator

A library providing a naive implementation of rogue-like dungeon generator
written in Rust

## About

This algorithm is a naive, un-optimized implementation used for demonstration
purposes. Use it at your own risk!

I made this project to learn Rust. This is my first ever program in Rust, and
it's poorly written, poorly designed but have been a great playground to do a
lot of mistakes and learn many aspects of Rust programming.

My intention was to learn with a funny project and practice procedural
generation as I'm learning the basics and try to dig deeper and deeper in the
language. For this purpose, I've designed my own dungeon-generator without using
any existing literacy in order to challenge my knowledges on procedural
generation.

As I was experimenting with Rust features, syntax and semantic, I may have
written some parts in an unconsistent way.

Please note that I'm not likely to maintain this project, but if you want to
report a bug or want to suggest a PR, please do so! I will look into it.

## Generation

The algorithm is quite naive but does its job, with some limitation. The output
of the generation is either a byte-array or a string with identifiers for floor,
doors, and walls.

### Rules

* rooms always have odd dimensions
* corridors always are aligned on even cells
* rooms always have at least 1 connection

### Procedures

1. set room dimensions
  a. if it's the first room, put it at 0, 0 in the dungeon
  b. if it's another room
    1. select an existing random room
    2. select a random direction
    3. try to put the room (check collision with all rooms in the dungeon)
    4. if fails, go back to 2., if all directions have been tested, go back to 1.
    5. put the room in the dungeon
2. create 1-4 connections with existing rooms
3. when all rooms have been created, resolves the connections

### Notes

The algorithm is far from being perfect, but have some advantages: it never
fails, so the computation time is quite easy to determine, and it's quite fast
even with a lot of rooms, thanks to the path-solving which is only checking
1-to-1 connections.

Of course, on average modern hardware, there is almost no gain and many other,
more clever and better written implementations would outperform this one.

A side-effect of this generation is that corridors will always be aligned on
even tiles of the grid.

## Integration

The lib adhere to the C calling convention and can be linked in other programs
if you mind it.

Look at the examples.

## Examples
### Rust

```
# run with default arguments
cargo run

# run with user arguments
cargo run -- --seed --rooms 5

# display list of arguments
cargo run -- --help
```

### C

```
# build
make

# run
./dungen

# clear files
make clean
```

## Example of generations

```
Map 42
..###########.............
..#...#...#.#.............
..oxxxoxx.#.#.............
..xxxxxxx.#.#.............
..xxxxxxx.#.oxxxx.........
..xxxxxxx.#.xxxxx.........
..xxxxxxx.#.xxxxx.........
..........#.xxxxx.........
..........#.xxxxx.........
..........#...............
###########...#####.......
#.........#...#...#.......
#.xxxxxxx.#.xxoxx.#.xxxxx.
#.xxxxxxx.#.xxxxx.#.xxxxx.
#.xxxxxxo##.xxxxo##.xxxxx.
#.xxxxxxx...xxxxx.#.xxxxx.
#.xxxxoxx...xxxxx.#.xxxxx.
#.....#.....xxxxx.#.xxxxx.
#.....#.....xxxxx.#.xxoxx.
#.....#...........#...#...
##oxxxxxx##############...
#.xxxxxxx.....#...#.......
#.xxxxxxx...xxoxx.#.......
#.xxxxxxx...xxxxx.#.......
#.xxxxxxx...xxxxx.#.......
#...........xxxxx.#.......
#...........xxxxx.#.......
#...........xxxxx.#.......
#...........oxxxx.#.......
#...........#.....#.......
###################.......
```

```
Map 30 
#############################........
#...........................#........
#.xxxxx.................#############
#.xxxxx.................#...#.......#
##oxxxx...............xxoxo##.......#
..xxxxx...............xxxxx.........#
..xxxxx...xxxxxxx.....xxxxx.........#
..xxxxx...xxxxxxx.....xxxxx.........#
..xxxxx...xxxxxxx.....xxxxx.........#
..........xxxxxxx...................#
..........oxxxxxx...................#
..........#.........................#
#################################...#
#.......#.........#.........#...#...#
#.xxxxx.#.xxxxxxx.#...xxxxx.##oxoxx.#
#.xxxxx.#.xxxxxxx.#...xxxxx.#.xxxxx.#
#.xxxxx.##oxxxxxx.#.##oxxxo##.xxxxx.#
#.xxxxx...xxxxxxx.#.#.xxxxx.#.xxxxx.#
##oxxxx...xxxxoxx.#.#.xxxxx.#.xxxxx.#
..xxxxx.......#...#.#.......#.xxxxx.#
..xxxxx.......#####.#.......#.oxxxx.#
....................#.......#.#.....#
....................#################
```
