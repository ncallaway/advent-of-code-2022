use itertools::Itertools;
use super::stack::CrateStack;
use regex::Regex;


pub fn solve(input: &str) -> u64 {
  let (stack_lines, instruction_lines) = input.split("\n\n").next_tuple().expect("Must have both a stack and instructions");
  let mut stacks = construct_starting_stack(stack_lines);
  apply_instructions(&mut stacks, instruction_lines);
  let tops = collect_tops(&stacks);

  println!("Top of the stacks: {}\n", tops);
  0
}

/// Construct the starting stack from the input. Returns a tuple
/// with the constructed stacks and the remaining input
fn construct_starting_stack(stack_lines: &str) -> Vec<CrateStack> {
  let mut stacks: Vec<CrateStack> = vec![];

  for stack_line in stack_lines.split('\n') {
    for (idx, char) in stack_line.chars().enumerate() {
      if (idx + 3) % 4 != 0 {
        continue;
      }
      let stack_idx = ((idx + 3) / 4) - 1;

      if stack_idx >= stacks.len() {
        stacks.push(CrateStack::new());
      }

      if char.is_alphabetic() {
        stacks[stack_idx].push(char);
      }
    }
  }

  // reverse all the stacks, since we built them top-down
  for stack in stacks.iter_mut() {
    stack.reverse();
  }

  stacks
}

fn apply_instructions(stacks: &mut Vec<CrateStack>, instructions: &str) {
  let instruction_lines = instructions.split('\n');

  for line in instruction_lines {
    apply_instruction(stacks, line)
  }
}

fn apply_instruction(stacks: &mut Vec<CrateStack>, line: &str) {
  let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
  let caps = re.captures(line).unwrap();


  let count = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
  let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
  let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

  // ensure from and to are valid
  if from >= stacks.len() || to >= stacks.len() {
    panic!("a line was invalid ({}). One of the indexes was out of bounds of the stacks. The stack size was: {}", line, stacks.len());
  }

  for _ in 0..count {
    let element = stacks[from].pop();
    if let Some(c) = element { stacks[to].push(c) }
  }
}

fn collect_tops(stacks: &[CrateStack]) -> String {
  let s: String = stacks.iter().map(|s| s.peek().unwrap_or(' ')).collect();

  s
}


#[cfg(test)]
mod tests {
  use super::*;

  const MINIMAL_STACKS: &str = r#"
    [D]
[N] [C]
 1   2"#;

const SAMPLE_STACKS: &str = r#"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3"#;

  #[test]
  fn a_sample_test() {
    assert_eq!(1, 1);
  }

  #[test]
  fn construct_minimal_stacks() {
    let stacks = construct_starting_stack(MINIMAL_STACKS);

    assert_eq!(stacks.len(), 2);
    assert_eq!(stacks[0].peek(), Some('N'));
    assert_eq!(stacks[0]._len(), 1);
    assert_eq!(stacks[1].peek(), Some('D'));
    assert_eq!(stacks[1]._len(), 2);

  }

  #[test]
  fn construct_sample_stacks() {
    let stacks = construct_starting_stack(SAMPLE_STACKS);

    assert_eq!(stacks.len(), 3);
    assert_eq!(stacks[0].peek(), Some('N'));
    assert_eq!(stacks[0]._len(), 2);
    assert_eq!(stacks[1].peek(), Some('D'));
    assert_eq!(stacks[1]._len(), 3);
    assert_eq!(stacks[2].peek(), Some('P'));
    assert_eq!(stacks[2]._len(), 1);
  }

  #[test]
  fn apply_instruction_to_minimal() {
    let mut stacks = construct_starting_stack(MINIMAL_STACKS);
    apply_instruction(&mut stacks, "move 2 from 2 to 1");
    assert_eq!(stacks[0].peek(), Some('C'));
    assert_eq!(stacks[0]._len(), 3);
    assert_eq!(stacks[1].peek(), None);
    assert_eq!(stacks[1]._len(), 0);
  }

  #[test]
  fn apply_instruction_to_sample() {
    let mut stacks = construct_starting_stack(SAMPLE_STACKS);
    apply_instruction(&mut stacks, "move 1 from 2 to 1");
    assert_eq!(stacks[0].peek(), Some('D'));
    assert_eq!(stacks[0]._len(), 3);
    assert_eq!(stacks[1].peek(), Some('C'));
    assert_eq!(stacks[1]._len(), 2);
    assert_eq!(stacks[2].peek(), Some('P'));
    assert_eq!(stacks[2]._len(), 1);

    apply_instruction(&mut stacks, "move 3 from 1 to 3");
    assert_eq!(stacks[0].peek(), None);
    assert_eq!(stacks[0]._len(), 0);
    assert_eq!(stacks[1].peek(), Some('C'));
    assert_eq!(stacks[1]._len(), 2);
    assert_eq!(stacks[2].peek(), Some('Z'));
    assert_eq!(stacks[2]._len(), 4);
  }

  #[test]
  fn test_collect_tops() {
    let stacks = construct_starting_stack(SAMPLE_STACKS);
    assert_eq!(collect_tops(&stacks), "NDP");
  }

  #[test]
  #[should_panic]
  fn apply_invalid_instructions() {
    let mut stacks = construct_starting_stack(MINIMAL_STACKS);
    apply_instruction(&mut stacks, "move 3 from 14 to 9");
  }
}

