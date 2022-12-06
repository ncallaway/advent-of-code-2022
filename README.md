# Advent of Code 2022

These are my solutions to the [Advent of Code 2022](https://adventofcode.com/2022) puzzles. These solutions aren't great, but they're mine, so that's something.

These are starting out in Rust, because I'm not very good at Rust.

## Getting Started

`cargo run -rq [day] [puzzle] [type]`

- `[day]` is one of `day-1 | day-2 | ...`
- `[puzzle]` is one of `puzzle-1 | puzzle-2`
- `[type]` is one of `sample | puzzle`, where `sample` runs with the sample input, and `puzzle` runs with the puzzle input


## Day 1

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

## Day 2

```sh
# puzzle-1
$ cargo run -rq day-2 puzzle-1 puzzle
(153.325µs)
Total score: 15691

$ cargo run -rq day-2 puzzle-2 puzzle
(156.497µs)
Total score: 12989
```

## Day 3

```sh
# puzzle-1
$ cargo run -rq day-3 puzzle-1 puzzle
(63.398µs)
Priority sum: 7811

$ cargo run -rq day-3 puzzle-2 puzzle
(61.386µs)
Priority sum: 2639
```
## Day 4

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

## Day 5

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

## Day 6

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