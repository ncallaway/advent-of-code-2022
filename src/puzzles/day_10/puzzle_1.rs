use super::computer::{Computer, Instruction};

pub fn solve(input: &str) -> u64 {
  let mut computer = Computer::new();
  let instructions: Vec<Instruction> = input.split('\n').filter_map(Instruction::parse).collect();
  let mut ip: usize = 0;

  let mut sum_strength: i64 = 0;

  loop {
    let next = if ip < instructions.len() { Some(instructions[ip]) } else { None };

    let advance = computer.start_cycle(next);
    if advance && ip >= instructions.len() {
      break;
    } else if advance {
      ip += 1;
    }

    if computer.cycle % 40 == 20 {
      sum_strength += computer.regx as i64 * computer.cycle as i64;
    }

    computer.end_cycle();
  }

  if sum_strength < 0 {
    println!("WARNING WARNING! SUM SIGNAL STRENGTH WAS NEGATIVE. ACTUAL VALUE IS: {}", sum_strength);
  }

  sum_strength as u64
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_with_sample() {
    let sample_path: std::path::PathBuf = ["input", "day-10", "sample"].iter().collect();
    let input = std::fs::read_to_string(sample_path).expect("Unable to read file");
    assert_eq!(solve(&input), 13140);
  }

  #[test]
  fn solve_with_sample_small() {
    let sample_path: std::path::PathBuf = ["input", "day-10", "sample-small"].iter().collect();
    let input = std::fs::read_to_string(sample_path).expect("Unable to read file");
    assert_eq!(solve(&input), 0);
  }

  #[test]
  fn solve_with_puzzle() {
    let puzzle_path: std::path::PathBuf = ["input", "day-10", "puzzle"].iter().collect();
    let input = std::fs::read_to_string(puzzle_path).expect("Unable to read file");
    assert_eq!(solve(&input), 13180);
  }
}
