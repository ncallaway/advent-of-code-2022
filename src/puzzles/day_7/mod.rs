use lazy_static::lazy_static;
use self::dir::DirectoryTree;
use regex::Regex;

mod dir;
pub mod puzzle_1;
pub mod puzzle_2;

lazy_static! {
  static ref FILE_RE: Regex = Regex::new(r"(\d+) .*").unwrap();
}

fn parse_tree(input: &str) -> DirectoryTree {
  // line can be cmd or output
  let mut tree = DirectoryTree::new();
  // let mut pwd_handle: usize = tree.root_handle();
  let mut pwd = vec![tree.root_handle()];
  let mut cmd: Option<Command> = None; // no active cmd

  for line in input.split('\n') {
    let pwd_handle = *pwd.last().unwrap();
    // if cmd, can be ls, where we should find all the files
    // or can be `cd` where we should find all the
    if line.starts_with("$ ") {
      // cmd
      cmd = parse_command(line);
      if let Some(Command::cd(dir)) = cmd {
        if dir == "/" {
          pwd.clear();
          pwd.push(tree.root_handle());
        } else if dir == ".." {
          pwd.pop();
          if pwd.is_empty() {
            pwd.push(tree.root_handle());
          }
        } else {
          pwd.push(tree.add_dir(dir, pwd_handle));
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

#[derive(PartialEq, Debug)]
#[allow(non_camel_case_types)]
enum Command<'a> {
  cd(&'a str),
  ls
}

#[cfg(test)]
mod tests {
  use super::*;

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

