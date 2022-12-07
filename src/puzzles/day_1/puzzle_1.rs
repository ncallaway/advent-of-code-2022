pub fn solve(input: &str) -> u64 {
  max_calories(input).into()
}

fn max_calories(calories: &str) -> u32 {
  sum_largest_elf(parse_elves(calories))
}

fn parse_elves(calories: &str) -> Vec<Vec<u32>> {
  let mut vec = Vec::new();
  let mut inner = Vec::new();
  let lines = calories.split('\n');

  for line in lines {
    if !line.is_empty() {
      inner.push(line.parse::<u32>().expect("A value could not be converted to a u32"))
    } else if !inner.is_empty() {
      vec.push(inner);
      inner = Vec::new();
    }
  }

  if !inner.is_empty() {
    vec.push(inner);
  }

  vec
}

fn sum_largest_elf(elves: Vec<Vec<u32>>) -> u32 {
  let total_kcals = elves.iter().map(|x| x.iter().sum::<u32>());

  total_kcals.max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_with_sample() {
      let sample_path: std::path::PathBuf = ["input", "day-1", "sample"].iter().collect();
      let input = std::fs::read_to_string(sample_path).expect("Unable to read file");
      assert_eq!(solve(&input), 24000);
    }

    #[test]
    fn solve_with_puzzle() {
      let puzzle_path: std::path::PathBuf = ["input", "day-1", "puzzle"].iter().collect();
      let input = std::fs::read_to_string(puzzle_path).expect("Unable to read file");
      assert_eq!(solve(&input), 69501);
    }

    #[test]
    fn max_calories_sums_basic_calories() {
        assert_eq!(max_calories("100\n200"), 300);
    }

    #[test]
    fn max_calories_sums_largest_calories() {
        assert_eq!(max_calories("100\n200\n\n50\n100"), 300);
    }
}
