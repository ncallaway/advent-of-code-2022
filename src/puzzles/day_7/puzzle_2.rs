use regex::Regex;
use lazy_static::lazy_static;
use super::dir::DirectoryTree;


lazy_static! {
    static ref FILE_RE: Regex = Regex::new(r"(\d+) .*").unwrap();
}

const DISK_SIZE: u64 = 70000000;
const UPDATE_SIZE_REQUIRED: u64 = 30000000;

pub fn solve(input: &str) -> u64 {
  let mut tree = parse_tree(input);

  let free_space = compute_free_space(&mut tree);
  let delete_threshhold = minimum_size_to_delete(free_space).unwrap();
  smallest_directory_above_threshhold(&mut tree, delete_threshhold)
}

fn parse_tree(input: &str) -> DirectoryTree {
  // line can be cmd or output
  let mut tree = DirectoryTree::new();
  let mut pwd_handle: usize = tree.root_handle();
  let mut cmd: Option<Command> = None; // no active cmd

  for line in input.split('\n') {
    // if cmd, can be ls, where we should find all the files
    // or can be `cd` where we should find all the
    if line.starts_with("$ ") {
      // cmd
      cmd = parse_command(line);
      if let Some(Command::cd(dir)) = cmd {
        if dir == "/" {
          pwd_handle = tree.root_handle();
        } else if dir == ".." {
          pwd_handle = tree.parent_handle(pwd_handle).unwrap_or(0);
        } else {
          pwd_handle = tree.add_dir(dir, pwd_handle);
        }
      }
      if Some(Command::ls) == cmd {
        tree.reset_files(pwd_handle);
      }
    } else {
      // cmd output
      if cmd == Some(Command::ls) {
        let caps_opt = FILE_RE.captures(line);
        if let Some(captures) = caps_opt {
          tree.add_file(pwd_handle, captures[1].parse::<u64>().unwrap());
        }
      }
    }
  }

  tree
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

#[derive(PartialEq, Debug)]
#[allow(non_camel_case_types)]
enum Command<'a> {
  cd(&'a str),
  ls
}

fn parse_command(line: &str) -> Option<Command> {
  if line.starts_with("$ ls") {
    Some(Command::ls)
  } else if line.starts_with("$ cd") {
    let dir = &line[5..];
    Some(Command::cd(dir))
  } else {
    None
  }
}

// fn parse_ls_output(line: &str) ->


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

  #[test]
  fn parse_command_with_cd() {
    assert_eq!(parse_command("$ cd /"), Some(Command::cd("/")));
    assert_eq!(parse_command("$ cd a"), Some(Command::cd("a")));
  }

  #[test]
  fn parse_command_with_ls() {
    assert_eq!(parse_command("$ ls"), Some(Command::ls));
  }

  #[test]
  fn parse_command_with_unknown() {
    assert_eq!(parse_command("$ foo"), None);
    assert_eq!(parse_command("ls"), None);
  }

  #[test]
  fn sample_test() {
    assert_eq!(1, 1);
  }
}

