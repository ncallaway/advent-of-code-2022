pub fn solve(input: &str) -> u64 {
  sum_priorities(input).into()
}


fn sum_priorities(rucksacks: &str) -> u32 {
  let mut lines = rucksacks.split('\n');
  let mut sum: u32 = 0;

  while let Some(first) = lines.next() {
      let second = lines.next().expect("Expected to have second elf in group");
      let third = lines.next().expect("Expected to have third elf in group");

      sum += find_duplicate_badge_priority(first, second, third) as u32
  }

  sum
}

fn find_duplicate_badge_priority(first: &str, second: &str, third: &str) -> u8 {
  let first_mask = compartment_mask(first);
  let second_mask = compartment_mask(second);
  let third_mask = compartment_mask(third);

  let shared = first_mask & second_mask & third_mask;
  mask_to_priority(shared)
}

fn compartment_mask(compartment: &str) -> u64 {
  let mut mask: u64 = 0;

  for c in compartment.chars() {
      let mask_idx = calculate_priority(c);
      mask |= 1 << mask_idx;
  }

  mask
}

fn mask_to_priority(mask: u64) -> u8 {
  mask.trailing_zeros() as u8
}

fn calculate_priority(char: char) -> u8 {
  let c = char as u8;

  if c >= 97 {
      return c-96; // 96 = 'a' ascii value
  }
  c-64+26 // 64 = 'A' ascii value, 26=a-z range
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_with_sample() {
    let sample_path: std::path::PathBuf = ["input", "day-3", "sample"].iter().collect();
    let input = std::fs::read_to_string(sample_path).expect("Unable to read file");
    assert_eq!(solve(&input), 70);
  }

  #[test]
  fn solve_with_puzzle() {
    let puzzle_path: std::path::PathBuf = ["input", "day-3", "puzzle"].iter().collect();
    let input = std::fs::read_to_string(puzzle_path).expect("Unable to read file");
    assert_eq!(solve(&input), 2639);
  }

  #[test]
  fn calculate_priority_sample() {
      assert_eq!(calculate_priority('a'), 1);
      assert_eq!(calculate_priority('z'), 26);
      assert_eq!(calculate_priority('A'), 27);
      assert_eq!(calculate_priority('Z'), 52);

      assert_eq!(calculate_priority('p'), 16);
      assert_eq!(calculate_priority('L'), 38);
  }
}
