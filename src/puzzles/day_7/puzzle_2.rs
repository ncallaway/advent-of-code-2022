use super::dir::DirectoryTree;
use super::parse_tree;

const DISK_SIZE: u64 = 70000000;
const UPDATE_SIZE_REQUIRED: u64 = 30000000;

pub fn solve(input: &str) -> u64 {
  let mut tree = parse_tree(input);

  let free_space = compute_free_space(&mut tree);
  let delete_threshhold = minimum_size_to_delete(free_space).unwrap();
  smallest_directory_above_threshhold(&mut tree, delete_threshhold)
}

fn compute_free_space(tree: &mut DirectoryTree) -> u64 {
  let total_used = tree.total_size(tree.root_handle());
  DISK_SIZE - total_used
}

fn minimum_size_to_delete(free_space: u64) -> Option<u64> {
  if free_space > UPDATE_SIZE_REQUIRED {
    return None;
  }

  Some(UPDATE_SIZE_REQUIRED - free_space)
}

fn smallest_directory_above_threshhold(tree: &mut DirectoryTree, threshhold: u64) -> u64 {
  let dirs: Vec<usize> = tree.get_all_dirs().collect();

  let mut minimum_size = tree.total_size(0);
  // let mut minimum_idx = 0;

  for dir in dirs {
    let size = tree.total_size(dir);
    if size > threshhold && size < minimum_size {
      minimum_size = size;
      // minimum_idx = dir;
    }
  }

  minimum_size
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_with_sample() {
    let sample_path: std::path::PathBuf = ["input", "day-7", "sample"].iter().collect();
    let input = std::fs::read_to_string(sample_path).expect("Unable to read file");
    assert_eq!(solve(&input), 24933642);
  }

  #[test]
  fn solve_with_puzzle() {
    let puzzle_path: std::path::PathBuf = ["input", "day-7", "puzzle"].iter().collect();
    let input = std::fs::read_to_string(puzzle_path).expect("Unable to read file");
    assert_eq!(solve(&input), 5756764);
  }

}

