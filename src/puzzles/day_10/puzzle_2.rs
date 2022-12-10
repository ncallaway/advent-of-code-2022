use super::computer::{Computer, Instruction};

pub fn solve(input: &str) -> u64 {
  let crt = run_with_crt(input);
  println!("{}", crt);
  0
}

fn run_with_crt(input: &str) -> String {
  let mut crt = String::from("----------------- CRT ------------------\n");

  let mut computer = Computer::new();
  let instructions: Vec<Instruction> = input.split('\n').filter_map(Instruction::parse).collect();
  let mut ip: usize = 0;

  loop {
    let next = if ip < instructions.len() { Some(instructions[ip]) } else { None };

    let advance = computer.start_cycle(next);
    if advance && ip >= instructions.len() {
      break;
    } else if advance {
      ip += 1;
    }

    print_crt_pixel(&mut crt, &computer);

    computer.end_cycle();
  }

  crt.push_str("\n--------------- END CRT ----------------");
  crt
}



fn print_crt_pixel(crt: &mut String, computer: &Computer) {
  let char = if is_lit(computer) { '#' } else { '.' };

  if computer.cycle > 1 && computer.cycle % 40 == 1 {
    crt.push('\n')
  }
  crt.push(char);
}

fn is_lit(computer: &Computer) -> bool {
  let col = (computer.cycle - 1) % 40;

  (col as i32 - computer.regx).abs() <= 1
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_with_sample() {
    let sample_path: std::path::PathBuf = ["input", "day-10", "sample"].iter().collect();
    let input = std::fs::read_to_string(sample_path).expect("Unable to read file");
    assert_eq!(run_with_crt(&input), "----------------- CRT ------------------
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
--------------- END CRT ----------------");
  }

  #[test]
  fn solve_with_puzzle() {
    let puzzle_path: std::path::PathBuf = ["input", "day-10", "puzzle"].iter().collect();
    let input = std::fs::read_to_string(puzzle_path).expect("Unable to read file");
    assert_eq!(run_with_crt(&input), "----------------- CRT ------------------
####.####.####..##..#..#...##..##..###..
#.......#.#....#..#.#..#....#.#..#.#..#.
###....#..###..#....####....#.#..#.###..
#.....#...#....#....#..#....#.####.#..#.
#....#....#....#..#.#..#.#..#.#..#.#..#.
####.####.#.....##..#..#..##..#..#.###..
--------------- END CRT ----------------");
  }



  #[test]
  fn is_lit_samples() {
    let mut computer = Computer { cycle: 1, regx: 1, running: None };
    assert!(is_lit(&computer));

    computer = Computer { cycle: 2, regx: 1, running: None };
    assert!(is_lit(&computer));

    computer = Computer { cycle: 3, regx: 16, running: None };
    assert!(!is_lit(&computer));

    computer = Computer { cycle: 4, regx: 16, running: None };
    assert!(!is_lit(&computer));

    computer = Computer { cycle: 5, regx: 5, running: None };
    assert!(is_lit(&computer));

    computer = Computer { cycle: 6, regx: 5, running: None };
    assert!(is_lit(&computer));
  }

}
