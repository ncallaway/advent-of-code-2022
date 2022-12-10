use std::collections::HashSet;

pub fn solve(input: &str) -> u64 {
  let set = run(input, 2);
  set.len() as u64
}


#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct RopePosition {
  x: i32,
  y: i32
}

pub fn run(input: &str, rope_size: usize) -> HashSet<RopePosition>{

  let mut rope = vec![RopePosition { x: 0, y: 0 }; rope_size];
  let mut tail_positions: HashSet<RopePosition> = HashSet::new();
  tail_positions.insert(rope[rope_size-1].clone());

  for line in input.split('\n') {
    let mut cmd = parse_command(line).expect("A command was not valid");

    while !cmd.is_done() {
      // apply command

      move_head(&mut rope[0], &cmd);
      for idx in 1..rope_size {
        let leader = rope[idx - 1].clone();
        move_tail(&mut rope[idx], leader);
      }

      tail_positions.insert(rope[rope_size-1].clone());
      cmd.reduce_amount();
    }
  }

  tail_positions
}

fn move_head(head: &mut RopePosition, cmd: &Command) {
  match cmd {
    Command::Right(_) => head.x += 1,
    Command::Left(_) => head.x -= 1,
    Command::Down(_) => head.y -= 1,
    Command::Up(_) => head.y += 1,
  }
}

fn move_tail(tail: &mut RopePosition, leader: RopePosition) {
  let x_delta = leader.x - tail.x;
  let y_delta = leader.y - tail.y;

  if x_delta.abs() <= 1 && y_delta.abs() <= 1 {
    // no change is necessary, because we are adjacent
    return;
  }

  tail.x += x_delta.signum();
  tail.y += y_delta.signum();
}


fn parse_command(line: &str) -> Option<Command> {
  let (cmd, amount) = line.split_once(' ')?;

  let amount = amount.parse::<u8>().unwrap();

  match cmd {
    "R" => Some(Command::Right(amount)),
    "L" => Some(Command::Left(amount)),
    "D" => Some(Command::Down(amount)),
    "U" => Some(Command::Up(amount)),
    _ => None
  }
}

#[derive(Debug, PartialEq)]
enum Command {
  Right(u8),
  Left(u8),
  Up(u8),
  Down(u8)
}

impl Command {
  fn amount(&self) -> u8 {
    match self {
      Command::Right(amt) | Command::Left(amt) | Command::Up(amt) | Command::Down(amt) => *amt,
    }
  }

  fn reduce_amount(&mut self) {
    let amount = self.amount();
    let updated_amount = if amount > 0 {amount - 1 } else { 0 };

    match self {
      Command::Right(amt) | Command::Left(amt) | Command::Up(amt) | Command::Down(amt) => *amt = updated_amount,
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
    let set = run("R 2\nD 2", 2);

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
    assert_eq!(parse_command("R 4"), Some(Command::Right(4)));
    assert_eq!(parse_command("L 2"), Some(Command::Left(2)));
    assert_eq!(parse_command("D 0"), Some(Command::Down(0)));
    assert_eq!(parse_command("U 5"), Some(Command::Up(5)));
  }

  #[test]
  fn test_cmd_amounts() {
    let mut right = Command::Right(4);
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
