use std::ops::Range;

pub fn solve(input: &str) -> u64 {
  count_enclosing_pairs(input).into()
}

fn count_enclosing_pairs(input: &str) -> u32 {
  let lines = input.split('\n');
  let mut enclosing_pairs = 0;

  for line in lines {
    if is_enclosing_pair(line) {
      enclosing_pairs += 1;
    }
  }

  enclosing_pairs
}

fn is_enclosing_pair(line: &str) -> bool {
  let mut pairs = line.split(',');
  let (left, right) = (pairs.next().expect("Need to have a left element in a line"),pairs.next().expect("Need to have a right element in a line"));

  let left_range = to_range(left);
  let right_range = to_range(right);

  left_encloses_right(&left_range, &right_range) || left_encloses_right(&right_range, &left_range)
}

fn left_encloses_right(left_range: &Range<u32>, right_range: &Range<u32>) -> bool {
  left_range.start <= right_range.start && left_range.end >= right_range.end
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
  fn is_enclosing_pair_true() {
    assert!(is_enclosing_pair("2-8,3-7"));
    assert!(is_enclosing_pair("3-7,2-8"));
    assert!(is_enclosing_pair("6-6,4-6"));
    assert!(is_enclosing_pair("4-6,6-6"));
  }

  #[test]
  fn is_enclosing_pair_false() {
    assert!(!is_enclosing_pair("2-4,6-8"));
    assert!(!is_enclosing_pair("2-6,4-8"));
    assert!(!is_enclosing_pair("2-3,4-5"));
    assert!(!is_enclosing_pair("7-9,5-7"));
  }

  #[test]
  fn test_left_encloses_right() {
    assert!(left_encloses_right(&(2..8), &(3..7)));
    assert!(!left_encloses_right(&(3..7), &(2..8)));

    assert!(!left_encloses_right(&(2..4), &(4..5)));

    assert!(left_encloses_right(&(4..6), &(6..6)));
    assert!(!left_encloses_right(&(6..6), &(4..6)));
  }
}