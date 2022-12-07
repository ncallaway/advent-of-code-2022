
use std::collections::HashMap;

pub struct DirectoryTree {
  all_directories: Vec<DirectoryNode>
}

impl DirectoryTree {
  pub fn new() -> DirectoryTree {
    DirectoryTree { all_directories: vec![DirectoryNode::new(None)] }
  }

  pub fn root_handle(&self) -> usize {
    0
  }

  pub fn get_all_dirs(&self) -> impl Iterator<Item=usize> + '_ {
    return self.all_directories.iter().enumerate().map(|(idx, _)| idx)
  }

  #[allow(dead_code)]
  pub fn get_name(&self, dir_handle: usize) -> Option<&str> {
    let dir_node = self.get_node(dir_handle);
    let parent_handle = dir_node.parent?;
    let parent_node = self.get_node(parent_handle);
    parent_node.get_child_name(dir_handle)
  }

  pub fn parent_handle(&self, dir_handle: usize) -> Option<usize> {
    let dir_node = self.get_node(dir_handle);
    dir_node.parent
  }

  pub fn add_file(&mut self, dir_handle: usize, file_size: u64) {
    let dir_node = self.get_node_mut(dir_handle);
    dir_node.files_size += file_size;
  }

  pub fn reset_files(&mut self, dir_handle: usize) {
    let dir_node = self.get_node_mut(dir_handle);
    dir_node.files_size = 0;
    dir_node.total_size = None;

    // todo, but not needed for advent of code:
    // to be correct this would also need to reset the total_size of all parents...
  }

  pub fn total_size(&mut self, dir_handle: usize) -> u64 {
    let dir_node = self.get_node(dir_handle);
    if let Some(total_size) = dir_node.total_size {
      return total_size;
    }

    let child_idxs: Vec<usize> = dir_node.children.values().copied().collect();

    let mut subdir_size = 0;
    {
      for child_idx in child_idxs {
        subdir_size += self.total_size(child_idx);
      }
    }
    let dir_node = self.get_node(dir_handle);
    let total_size = subdir_size + dir_node.files_size;

    let mut_dir_node = self.get_node_mut(dir_handle);
    mut_dir_node.total_size = Some(total_size);
    total_size
  }

  pub fn add_dir(&mut self, dir: &str, parent_handle: usize) -> usize {
    // check if parent already has directory
    {
      let parent_node = self.get_node(parent_handle);
      let child_handle_opt = parent_node.children.get(dir);
      if let Some(child_handle) = child_handle_opt{
        return *child_handle;
      }
    }

    self.all_directories.push(DirectoryNode::new(Some(parent_handle)));
    let child_handle = self.all_directories.len() - 1;
    let parent_node = self.get_node_mut(parent_handle);
    parent_node.add_child(dir, child_handle);
    child_handle
  }

  fn get_node(&self, idx: usize) -> &DirectoryNode {
    &self.all_directories[idx]
  }

  fn get_node_mut(&mut self, idx: usize) -> &mut DirectoryNode {
    &mut self.all_directories[idx]
  }
}

struct DirectoryNode {
  files_size: u64,
  total_size: Option<u64>,
  parent: Option<usize>,
  children: HashMap<String, usize>,
}

impl DirectoryNode {
  fn new(parent: Option<usize>)-> DirectoryNode {
    DirectoryNode { files_size: 0, total_size: None, parent, children: HashMap::new() }
  }

  // fn get_child_idx(&self, dir: &str) -> Option<usize> {
  //   self.children.get(dir).copied()
  // }

  fn add_child(&mut self, dir: &str, child_idx: usize) {
    self.children.insert(dir.to_string(), child_idx);
  }

  #[allow(dead_code)]
  fn get_child_name(&self, child_idx: usize) -> Option<&str> {
    self.children.iter()
      .find_map(|(k,v)| if *v == child_idx { Some(k) } else { None } )
      .map(|x| &**x)
  }

  // fn add_file(&mut self, file_size: u64) {
  //   self.files_size += file_size;
  // }

  // fn reset_files(&mut self) {
  //   self.files_size = 0;
  // }

  // fn total_size(&self, tree: &DirectoryTree) -> u64 {
  //   self.children.values().map(|dir_idx| {
  //     let dir = &tree.all_directories[*dir_idx];
  //     dir.total_size(tree)
  //   }).sum::<u64>() + self.files_size
  // }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn add_files_works() {
    let mut tree = DirectoryTree::new();
    let root = tree.root_handle();
    tree.add_file(root, 10);
    tree.add_file(root, 20);
    tree.add_file(root, 30);

    assert_eq!(tree.total_size(root), 60);

    tree.reset_files(root);
    tree.add_file(root, 10);
    assert_eq!(tree.total_size(root), 10);
  }

  #[test]
  fn add_child_directory() {
    let mut tree = DirectoryTree::new();
    let root = tree.root_handle();
    let a = tree.add_dir("a", 0);
    let b = tree.add_dir("b", 0);

    tree.add_file(root, 10);
    tree.add_file(a, 20);
    tree.add_file(b, 30);

    assert_eq!(tree.total_size(a), 20);
    assert_eq!(tree.total_size(b), 30);
    assert_eq!(tree.total_size(root), 60);
  }

  #[test]
  fn get_child_names() {
    let mut tree = DirectoryTree::new();
    let root = tree.root_handle();
    let a = tree.add_dir("a", 0);
    let b = tree.add_dir("b", 0);

    assert_eq!(tree.get_name(a), Some("a"));
    assert_eq!(tree.get_name(b), Some("b"));
    assert_eq!(tree.get_name(root), None);
  }
}
