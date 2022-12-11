use super::monkey::run_rounds;
use super::monkey::parser::parse_monkeys;

pub fn solve(input: &str) -> u64 {
  let mut monkeys = parse_monkeys(input);

  run_rounds(&mut monkeys, 20, |worry: u64| worry / 3)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_with_sample() {
    let sample_path: std::path::PathBuf = ["input", "day-11", "sample"].iter().collect();
    let input = std::fs::read_to_string(sample_path).expect("Unable to read file");
    assert_eq!(solve(&input), 10605);
  }

  #[test]
  fn solve_with_puzzle() {
    let puzzle_path: std::path::PathBuf = ["input", "day-11", "puzzle"].iter().collect();
    let input = std::fs::read_to_string(puzzle_path).expect("Unable to read file");
    assert_eq!(solve(&input), 78678);
  }
}
