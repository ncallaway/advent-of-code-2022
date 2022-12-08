use super::dir::DirectoryTree;
use super::parse_tree;

const DIRECTORY_LIMIT: u64 = 100000;

pub fn solve(input: &str) -> u64 {
  let mut tree = parse_tree(input);

  sum_tree(&mut tree)
}

fn sum_tree(tree: &mut DirectoryTree) -> u64 {
  let mut sum = 0;
  let dirs: Vec<usize> = tree.get_all_dirs().collect();
  for dir in dirs {
    let total = tree.total_size(dir);
    if total <= DIRECTORY_LIMIT {
      sum += total;
    }
  }
  sum
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_with_sample() {
    let sample_path: std::path::PathBuf = ["input", "day-7", "sample"].iter().collect();
    let input = std::fs::read_to_string(sample_path).expect("Unable to read file");
    assert_eq!(solve(&input), 95437);
  }

  #[test]
  fn solve_with_puzzle() {
    let puzzle_path: std::path::PathBuf = ["input", "day-7", "puzzle"].iter().collect();
    let input = std::fs::read_to_string(puzzle_path).expect("Unable to read file");
    assert_eq!(solve(&input), 1297683);
  }
}

