use super::monkey::run_rounds;
use super::monkey::parser::parse_monkeys;

pub fn solve(input: &str) -> u64 {
  let mut monkeys = parse_monkeys(input);
  let mut combined_divisors = 1;
  for m in monkeys.iter() {
    combined_divisors *= m.throw_conditional.divisor;
  }

  run_rounds(&mut monkeys, 10_000, |worry: u64| worry % combined_divisors)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_with_sample() {
    let sample_path: std::path::PathBuf = ["input", "day-11", "sample"].iter().collect();
    let input = std::fs::read_to_string(sample_path).expect("Unable to read file");
    assert_eq!(solve(&input), 2713310158);
  }

  #[test]
  fn solve_with_puzzle() {
    let puzzle_path: std::path::PathBuf = ["input", "day-11", "puzzle"].iter().collect();
    let input = std::fs::read_to_string(puzzle_path).expect("Unable to read file");
    assert_eq!(solve(&input), 15333249714);
  }
}
