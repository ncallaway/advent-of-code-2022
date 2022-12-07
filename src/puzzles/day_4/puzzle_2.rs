use std::ops::Range;

pub fn solve(input: &str) -> u64 {
  count_overlapping_pairs(input).into()
}

fn count_overlapping_pairs(input: &str) -> u32 {
  let lines = input.split('\n');
  let mut enclosing_pairs = 0;

  for line in lines {
    if is_overlapping_pair(line) {
      enclosing_pairs += 1;
    }
  }

  enclosing_pairs
}

fn is_overlapping_pair(line: &str) -> bool {
  let mut pairs = line.split(',');
  let (left, right) = (pairs.next().expect("Need to have a left element in a line"),pairs.next().expect("Need to have a right element in a line"));

  let left_range = to_range(left);
  let right_range = to_range(right);

  overlaps(&left_range, &right_range)
}

fn overlaps(a: &Range<u32>, b: &Range<u32>) -> bool {
  let no_overlap = (b.start > a.end && b.end > a.end) ||(b.start < a.start && b.end < a.start);
  !no_overlap
}

fn to_range(str_range: &str) -> Range<u32> {
  let mut pairs = str_range.split('-');
  let (left, right) = (pairs.next().expect("Need to have a left element in a range"),pairs.next().expect("Need to have a right element in a range"));
  let l = left.parse::<u32>().unwrap();
  let r = right.parse::<u32>().unwrap();

  l..r
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_with_sample() {
    let sample_path: std::path::PathBuf = ["input", "day-4", "sample"].iter().collect();
    let input = std::fs::read_to_string(sample_path).expect("Unable to read file");
    assert_eq!(solve(&input), 4);
  }

  #[test]
  fn solve_with_puzzle() {
    let puzzle_path: std::path::PathBuf = ["input", "day-4", "puzzle"].iter().collect();
    let input = std::fs::read_to_string(puzzle_path).expect("Unable to read file");
    assert_eq!(solve(&input), 837);
  }

  #[test]
  fn is_overlapping_pair_true() {
    assert!(is_overlapping_pair("2-8,3-7"));
    assert!(is_overlapping_pair("5-7,7-9"));
    assert!(is_overlapping_pair("6-6,4-6"));
    assert!(is_overlapping_pair("2-6,4-8"));
  }

  #[test]
  fn is_overlapping_pair_false() {
    assert!(!is_overlapping_pair("2-4,6-8"));
    assert!(!is_overlapping_pair("2-3,4-5"));
  }

  #[test]
  fn test_left_encloses_right() {
    assert!(overlaps(&(2..8), &(3..7)));

    assert!(overlaps(&(2..4), &(4..5)));
    assert!(overlaps(&(2..6), &(4..8)));

    assert!(overlaps(&(4..6), &(6..6)));

    assert!(!overlaps(&(2..4), &(5..6)));
  }
}