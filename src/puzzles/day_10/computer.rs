pub struct Computer {
  pub cycle: u32,
  pub regx: i32,
  pub running: Option<InstructionExecution>
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Instruction {
  Addx(i32),
  Noop
}

#[derive(PartialEq, Eq)]
pub struct InstructionExecution {
  instruction: Instruction,
  start_cycle: u32,
}

impl Computer {
  pub fn new() -> Computer {
    Computer {
      cycle: 1,
      regx: 1,
      running: None
    }
  }

  pub fn start_cycle(&mut self, next_instruction: Option<Instruction>) -> bool {
    if self.running.is_none() {
      self.running = next_instruction.map(|i| InstructionExecution { instruction: i, start_cycle: self.cycle});
      true
    } else {
      false
    }
  }

  pub fn end_cycle(&mut self) {
    self.cycle += 1;

    // check if instruction has finished
    if let Some(execution) = &self.running {
      match execution.instruction {
        Instruction::Addx(amt) => {
          if self.cycle >= (execution.start_cycle + 2) {
            self.running = None;
            self.regx += amt;
          }
        },
        Instruction::Noop => {
          self.running = None;
        }
      }
    }
  }
}

impl Instruction {
  pub fn parse(command: &str) -> Option<Instruction> {
    if command.starts_with("noop") {
      return Some(Instruction::Noop);
    }

    if command.starts_with("addx") {
      let (_, arg) = command.split_once(' ')?;
      let arg = arg.parse::<i32>().ok()?;
      return Some(Instruction::Addx(arg));
    }

    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse() {
    assert_eq!(Instruction::parse("noop"), Some(Instruction::Noop));
    assert_eq!(Instruction::parse("addx 3"), Some(Instruction::Addx(3)));
    assert_eq!(Instruction::parse("addx -5"), Some(Instruction::Addx(-5)));

    assert_eq!(Instruction::parse("fizzbuzz"), None);
    assert_eq!(Instruction::parse("addx fizzbuzz"), None);
    assert_eq!(Instruction::parse("addx"), None);
  }


  #[test]
  fn test_computer_noops() {
    let mut computer = Computer::new();
    assert_eq!(computer.cycle, 1);
    assert_eq!(computer.regx, 1);

    assert!(computer.start_cycle(Some(Instruction::Noop)));
    assert_eq!(computer.cycle, 1);
    assert_eq!(computer.regx, 1);
    computer.end_cycle();
    assert_eq!(computer.cycle, 2);
    assert_eq!(computer.regx, 1);

    assert!(computer.start_cycle(Some(Instruction::Noop)));
    assert_eq!(computer.cycle, 2);
    assert_eq!(computer.regx, 1);
    computer.end_cycle();
    assert_eq!(computer.cycle, 3);
    assert_eq!(computer.regx, 1);
  }

  #[test]
  fn test_computer_sample_small() {
    let mut computer = Computer::new();

    assert!(computer.start_cycle(Some(Instruction::Noop)));
    assert_eq!(computer.cycle, 1);
    assert_eq!(computer.regx, 1);
    computer.end_cycle();
    assert_eq!(computer.cycle, 2);
    assert_eq!(computer.regx, 1);

    assert!(computer.start_cycle(Some(Instruction::Addx(3))));
    assert_eq!(computer.cycle, 2);
    assert_eq!(computer.regx, 1);
    computer.end_cycle();
    assert_eq!(computer.cycle, 3);
    assert_eq!(computer.regx, 1);

    assert!(!computer.start_cycle(Some(Instruction::Addx(-5))));
    assert_eq!(computer.cycle, 3);
    assert_eq!(computer.regx, 1);
    computer.end_cycle();
    assert_eq!(computer.cycle, 4);
    assert_eq!(computer.regx, 4);

    assert!(computer.start_cycle(Some(Instruction::Addx(-5))));
    assert_eq!(computer.cycle, 4);
    assert_eq!(computer.regx, 4);
    computer.end_cycle();
    assert_eq!(computer.cycle, 5);
    assert_eq!(computer.regx, 4);

    assert!(!computer.start_cycle(None));
    assert_eq!(computer.cycle, 5);
    assert_eq!(computer.regx, 4);
    computer.end_cycle();
    assert_eq!(computer.cycle, 6);
    assert_eq!(computer.regx, -1);
  }
}