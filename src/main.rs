mod puzzles;

use std::{env, fs};
use std::time::Instant;
use std::path::PathBuf;

type Solver = fn(&str) -> u32;

fn main() {
  let start = Instant::now();

  let day = env::args().nth(1).expect("A day (day-1|day-2|...) must be provided");
  let puzzle = env::args().nth(2).expect("A puzzle (puzzle-1|puzzle-2) must be provided");
  let t = env::args().nth(3).expect("A type (sample|puzzle) must be provided");

  let input_path: PathBuf = ["input", &day, &t].iter().collect();
  let input = fs::read_to_string(input_path).expect("Unable to read file");

  let solver = get_solver(&day, &puzzle).unwrap_or_else(|| panic!("No solver found for {} {}", day, puzzle));
  let result = solver(&input);
  let msg = get_solution_message(&day, &puzzle);

  let duration = start.elapsed();

  println!("({:?})\n{}: {}", duration, msg, result);
}

fn get_solution_message(day: &str, puzzle: &str) -> &'static str {
  match (day, puzzle) {
    ("day-1", "puzzle-1") => "Total calories",
    _ => "Solution"
  }
}

fn get_solver(day: &str, puzzle: &str) -> Option<Solver> {
  match (day, puzzle) {
    ("day-1", "puzzle-1") => Some(puzzles::day_1::puzzle_1::solve),
    ("day-1", "puzzle-2") => Some(puzzles::day_1::puzzle_2::solve),
    _ => None
  }
}