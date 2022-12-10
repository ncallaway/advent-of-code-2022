# Advent of Code 2022

These are my solutions to the [Advent of Code 2022](https://adventofcode.com/2022) puzzles. These solutions aren't great, but they're mine, so that's something.

These are starting out in Rust, because I'm not very good at Rust.

## Getting Started

`cargo run -rq [day] [puzzle] [type]`

- `[day]` is one of `day-1 | day-2 | ...`
- `[puzzle]` is one of `puzzle-1 | puzzle-2`
- `[type]` is one of `sample | puzzle`, where `sample` runs with the sample input, and `puzzle` runs with the puzzle input

## Notes

- Day 8, Puzzle 2: I'm not thrilled with the quality of this. There's a lot of copy and paste code, feels like it could be a lot cleaner.

## Puzzles
### Day 1

```sh
# puzzle-1
$ cargo run -rq day-1 puzzle-1 puzzle
(66.736µs)
Total calories: 69501

# puzzle-2
$ cargo run -rq day-1 puzzle-2 puzzle
(78.505µs)
Solution: 202346
```

### Day 2

```sh
# puzzle-1
$ cargo run -rq day-2 puzzle-1 puzzle
(153.325µs)
Total score: 15691

$ cargo run -rq day-2 puzzle-2 puzzle
(156.497µs)
Total score: 12989
```

### Day 3

```sh
# puzzle-1
$ cargo run -rq day-3 puzzle-1 puzzle
(63.398µs)
Priority sum: 7811

$ cargo run -rq day-3 puzzle-2 puzzle
(61.386µs)
Priority sum: 2639
```
### Day 4

```sh
# puzzle-1
$ cargo run -rq day-4 puzzle-1 puzzle
(91.328µs)
Pairs with one range enclosing the other: 450

# puzzle-2
$ cargo run -rq day-4 puzzle-2 puzzle
(84.948µs)
Pairs with one range enclosing the other: 837
```

### Day 5

```sh
# puzzle-1
$ cargo run -rq day-5 puzzle-1 puzzle
Top of the stacks: ZBDRNPMVH

(33.243852ms)
(This line is useless, read the previous line): 0

# puzzle-2
$ cargo run -rq day-5 puzzle-2 puzzle
Top of the stacks: WDLPFNNNB

(19.79369ms)
(This line is useless, read the previous line): 0
```

### Day 6

```sh
# puzzle-1
$ cargo run -rq day-6 puzzle-1 puzzle
(19.588µs)
Character Position: 1109

# puzzle-2
$ cargo run -rq day-6 puzzle-2 puzzle
(95.046µs)
Character Position: 3965
```
### Day 7

```sh
#puzzle-1
$ cargo run -rq day-7 puzzle-1 puzzle
(330.708µs)
Sum Dir Size: 1297683

#puzzle-2
$ cargo run -rq day-7 puzzle-2 puzzle
(354.67µs)
Sum Dir Size: 5756764
```

### Day 8

```sh
#puzzle-1
$ cargo run -rq day-8 puzzle-1 puzzle
(139.021µs)
Trees: 1698

#puzzle-2
$ cargo run -rq day-8 puzzle-2 puzzle
(674.857µs)
Trees: 672280
```

### Day 9

```sh
#puzzle-1
$ cargo run -rq day-9 puzzle-1 puzzle
(561.962µs)
Tail Positions: 5858

#puzzle-2$ cargo run -rq day-9 puzzle-2 puzzle
(612.903µs)
Tail Positions: 2602
```

### Day 10

```sh
#puzzle-1
$ cargo run -rq day-10 puzzle-1 puzzle
(21.6µs)
Sum Signal Strength: 13180

#puzzle-2
$ cargo run -rq day-10 puzzle-2 puzzle
----------------- CRT ------------------
#### #### ####  ##  #  #   ##  ##  ###
#       # #    #  # #  #    # #  # #  #
###    #  ###  #    ####    # #  # ###
#     #   #    #    #  #    # #### #  #
#    #    #    #  # #  # #  # #  # #  #
#### #### #     ##  #  #  ##  #  # ###
--------------- END CRT ----------------
(31.807µs)
Sum Signal Strength: 0
```


### Day 11

```sh
#puzzle-1

#puzzle-2

```

