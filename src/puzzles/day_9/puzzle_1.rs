use std::collections::HashSet;

pub fn solve(input: &str) -> u64 {
  let set = run(input);
  set.len() as u64
}

fn run(input: &str) -> HashSet<RopePosition>{
  let mut head = RopePosition { x: 0, y: 0 };
  let mut tail = RopePosition { x: 0, y: 0 };
  let mut tail_positions: HashSet<RopePosition> = HashSet::new();

  tail_positions.insert(tail.clone());

  for line in input.split('\n') {
    let mut cmd = parse_command(line).expect("A command was not valid");

    while !cmd.is_done() {
      // apply command

      move_head(&mut head, &cmd);
      move_tail(&mut tail, &head);

      tail_positions.insert(tail.clone());
      cmd.reduce_amount();
    }
  }

  tail_positions
}

fn move_head(head: &mut RopePosition, cmd: &Command) {
  match cmd {
    Command::RIGHT(_) => head.x += 1,
    Command::LEFT(_) => head.x -= 1,
    Command::DOWN(_) => head.y -= 1,
    Command::UP(_) => head.y += 1,
  }
}

fn move_tail(tail: &mut RopePosition, head: &RopePosition) {
  let x_delta = head.x - tail.x;
  let y_delta = head.y - tail.y;

  if x_delta.abs() <= 1 && y_delta.abs() <= 1 {
    // no change is necessary, because we are adjacent
    return;
  }

  tail.x += x_delta.signum();
  tail.y += y_delta.signum();
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct RopePosition {
  x: i32,
  y: i32
}

fn parse_command(line: &str) -> Option<Command> {
  let (cmd, amount) = line.split_once(' ')?;

  let amount = amount.parse::<u8>().unwrap();

  match cmd {
    "R" => Some(Command::RIGHT(amount)),
    "L" => Some(Command::LEFT(amount)),
    "D" => Some(Command::DOWN(amount)),
    "U" => Some(Command::UP(amount)),
    _ => None
  }
}

#[derive(Debug, PartialEq)]
enum Command {
  RIGHT(u8),
  LEFT(u8),
  UP(u8),
  DOWN(u8)
}

impl Command {
  fn amount(&self) -> u8 {
    match self {
      Command::RIGHT(amt) | Command::LEFT(amt) | Command::UP(amt) | Command::DOWN(amt) => *amt,
    }
  }

  fn reduce_amount(&mut self) {
    let amount = self.amount();
    let updated_amount = if amount > 0 {amount - 1 } else { 0 };

    match self {
      Command::RIGHT(amt) | Command::LEFT(amt) | Command::UP(amt) | Command::DOWN(amt) => *amt = updated_amount,
    }
  }

  fn is_done(&self) -> bool {
    self.amount() == 0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_with_sample() {
    let sample_path: std::path::PathBuf = ["input", "day-9", "sample"].iter().collect();
    let input = std::fs::read_to_string(sample_path).expect("Unable to read file");
    assert_eq!(solve(&input), 13);
  }

  #[test]
  fn solve_with_puzzle() {
    let puzzle_path: std::path::PathBuf = ["input", "day-9", "puzzle"].iter().collect();
    let input = std::fs::read_to_string(puzzle_path).expect("Unable to read file");
    assert_eq!(solve(&input), 5858);
  }

  #[test]
  fn test_run() {
    let set = run("R 2\nD 2");

    let expected = vec![
      RopePosition { x: 0, y: 0 },
      RopePosition { x: 1, y: 0 },
      RopePosition { x: 2, y: -1 }
    ];
    let expected_set = HashSet::from_iter(expected.iter().cloned());
    assert_eq!(set, expected_set)
  }

  #[test]
  fn test_parse_command() {
    assert_eq!(parse_command("R 4"), Some(Command::RIGHT(4)));
    assert_eq!(parse_command("L 2"), Some(Command::LEFT(2)));
    assert_eq!(parse_command("D 0"), Some(Command::DOWN(0)));
    assert_eq!(parse_command("U 5"), Some(Command::UP(5)));
  }

  #[test]
  fn test_cmd_amounts() {
    let mut right = Command::RIGHT(4);
    assert_eq!(right.amount(), 4);

    right.reduce_amount();
    assert_eq!(right.amount(), 3);

    right.reduce_amount();
    assert_eq!(right.amount(), 2);

    right.reduce_amount();
    assert_eq!(right.amount(), 1);
    assert!(!right.is_done());

    right.reduce_amount();
    assert_eq!(right.amount(), 0);

    right.reduce_amount();
    assert_eq!(right.amount(), 0);
    assert!(right.is_done())
  }
}
