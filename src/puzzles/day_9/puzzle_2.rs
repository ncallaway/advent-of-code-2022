use super::puzzle_1::run;

pub fn solve(input: &str) -> u64 {
  let set = run(input, 10);
  set.len() as u64
}


// #[derive(Hash, Eq, PartialEq, Debug, Clone)]
// struct RopePosition {
//   x: i32,
//   y: i32
// }



// fn run(input: &str, rope_size: usize) -> HashSet<RopePosition>{

//   let mut rope = vec![RopePosition { x: 0, y: 0 }; rope_size];
//   let mut tail_positions: HashSet<RopePosition> = HashSet::new();
//   tail_positions.insert(rope[rope_size-1].clone());

//   for line in input.split('\n') {
//     let mut cmd = parse_command(line).expect("A command was not valid");

//     while !cmd.is_done() {
//       // apply command

//       move_head(&mut rope[0], &cmd);
//       for idx in 1..rope_size {
//         let leader = rope[idx - 1].clone();
//         move_tail(&mut rope[idx], leader);
//       }

//       tail_positions.insert(rope[rope_size-1].clone());
//       cmd.reduce_amount();
//     }
//   }

//   tail_positions
// }

// fn move_head(head: &mut RopePosition, cmd: &Command) {
//   match cmd {
//     Command::RIGHT(_) => head.x += 1,
//     Command::LEFT(_) => head.x -= 1,
//     Command::DOWN(_) => head.y -= 1,
//     Command::UP(_) => head.y += 1,
//   }
// }

// fn move_tail(tail: &mut RopePosition, leader: RopePosition) {
//   let x_delta = leader.x - tail.x;
//   let y_delta = leader.y - tail.y;

//   if x_delta.abs() <= 1 && y_delta.abs() <= 1 {
//     // no change is necessary, because we are adjacent
//     return;
//   }

//   tail.x += x_delta.signum();
//   tail.y += y_delta.signum();
// }


// fn parse_command(line: &str) -> Option<Command> {
//   let (cmd, amount) = line.split_once(' ')?;

//   let amount = amount.parse::<u8>().unwrap();

//   match cmd {
//     "R" => Some(Command::RIGHT(amount)),
//     "L" => Some(Command::LEFT(amount)),
//     "D" => Some(Command::DOWN(amount)),
//     "U" => Some(Command::UP(amount)),
//     _ => None
//   }
// }

// #[derive(Debug, PartialEq)]
// enum Command {
//   RIGHT(u8),
//   LEFT(u8),
//   UP(u8),
//   DOWN(u8)
// }

// impl Command {
//   fn amount(&self) -> u8 {
//     match self {
//       Command::RIGHT(amt) | Command::LEFT(amt) | Command::UP(amt) | Command::DOWN(amt) => *amt,
//     }
//   }

//   fn reduce_amount(&mut self) {
//     let amount = self.amount();
//     let updated_amount = if amount > 0 {amount - 1 } else { 0 };

//     match self {
//       Command::RIGHT(amt) | Command::LEFT(amt) | Command::UP(amt) | Command::DOWN(amt) => *amt = updated_amount,
//     }
//   }

//   fn is_done(&self) -> bool {
//     self.amount() == 0
//   }
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_with_sample() {
    let sample_path: std::path::PathBuf = ["input", "day-9", "sample"].iter().collect();
    let input = std::fs::read_to_string(sample_path).expect("Unable to read file");
    assert_eq!(solve(&input), 1);
  }

  #[test]
  fn solve_with_second_sample() {
    let sample_path: std::path::PathBuf = ["input", "day-9", "sample-2"].iter().collect();
    let input = std::fs::read_to_string(sample_path).expect("Unable to read file");
    assert_eq!(solve(&input), 36);
  }

  #[test]
  fn solve_with_puzzle() {
    let puzzle_path: std::path::PathBuf = ["input", "day-9", "puzzle"].iter().collect();
    let input = std::fs::read_to_string(puzzle_path).expect("Unable to read file");
    assert_eq!(solve(&input), 2602);
  }
}
