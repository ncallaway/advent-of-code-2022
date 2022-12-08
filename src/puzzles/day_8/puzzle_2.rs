pub fn solve(input: &str) -> u64 {
  let grid = initialize_grid(input);
  let left_grid = left_trees(&grid);
  let right_grid = right_trees(&grid);
  let top_grid = top_trees(&grid);
  let bottom_grid = bottom_trees(&grid);

  find_max_tree(&left_grid, &right_grid, &top_grid, &bottom_grid) as u64
}

fn find_max_tree(left_grid: &Grid, right_grid: &Grid, top_grid: &Grid, bottom_grid: &Grid) -> u32 {
  left_grid.grid.iter().enumerate().map(|(idx, left)| {
    let left = *left as u32;
    let right = right_grid.grid[idx] as u32;
    let top = top_grid.grid[idx] as u32;
    let bottom = bottom_grid.grid[idx] as u32;

    left * right * top * bottom
  }).max().unwrap()
}

// grid vec
// 0 => row 0, col 0
// 1 => row 0, col 1
// ...
// w-1 => row 0, col w-1,
// w*R+C => row R, col C
struct Grid {
  width: usize,
  height: usize,
  grid: Vec<u8>
}

impl Grid {
  fn index(&self, row: usize, col: usize) -> usize {
    self.width*row + col
  }

  fn value(&self, row: usize, col: usize) -> u8 {
    self.grid[self.index(row, col)]
  }

  fn set_value(&mut self, value: u8, row: usize, col: usize) {
    let idx = self.index(row, col);
    self.grid[idx] = value;
  }
}

fn initialize_grid(input: &str) -> Grid {
  let mut width = 0;
  let mut height = 0;
  let mut grid: Vec<u8> = Vec::new();

  for line in input.split('\n') {
    if width == 0 { width = line.len(); }
    height += 1;

    for c in line.chars() {
      let value: u8 = c as u8 - 48; // 48 is '0'in ascii
      grid.push(value);
    }
  }

  Grid { width, height, grid }
}

fn top_trees(grid: &Grid) -> Grid {
  let mut top_grid = Grid { width: grid.width, height: grid.height, grid: vec![0; grid.width*grid.height] };
  let mut last_seen: Vec<Option<u8>> = vec![None; 10];

  for col in 0..grid.width {
    for item in &mut last_seen { *item = None; }

    for row in 0..grid.height {
      let cell = grid.value(row, col);
      let last_row = (cell as usize..10).filter_map(|idx| last_seen[idx]).max().unwrap_or(0);
      let distance = row as u8 - last_row;

      top_grid.set_value(distance, row, col);
      last_seen[cell as usize] = Some(row as u8);
    }
  }
  top_grid
}

fn bottom_trees(grid: &Grid) -> Grid {
  let mut bottom_grid = Grid { width: grid.width, height: grid.height, grid: vec![0; grid.width*grid.height] };
  let mut last_seen: Vec<Option<u8>> = vec![None; 10];

  for col in 0..grid.width {
    for item in &mut last_seen { *item = None; }

    for row in (0..grid.height).rev() {
      let cell = grid.value(row, col);
      let last_row = (cell as usize..10).filter_map(|idx| last_seen[idx]).min().unwrap_or(grid.height as u8 - 1);
      let distance = last_row - row as u8;

      bottom_grid.set_value(distance, row, col);
      last_seen[cell as usize] = Some(row as u8);
    }
  }
  bottom_grid
}

fn left_trees(grid: &Grid) -> Grid {
  let mut left_grid = Grid { width: grid.width, height: grid.height, grid: vec![0; grid.width*grid.height] };
  let mut last_seen: Vec<Option<u8>> = vec![None; 10];

  for row in 0..grid.height {
    for item in &mut last_seen { *item = None; }
    for col in 0..grid.width {
      let cell = grid.value(row, col);
      let last_col = (cell as usize..10).filter_map(|idx| last_seen[idx]).max().unwrap_or(0);
      let distance = col as u8 - last_col;

      left_grid.set_value(distance, row, col);
      last_seen[cell as usize] = Some(col as u8);
    }
  }
  left_grid
}


fn right_trees(grid: &Grid) -> Grid {
  let mut right_grid = Grid { width: grid.width, height: grid.height, grid: vec![0; grid.width*grid.height] };
  let mut last_seen: Vec<Option<u8>> = vec![None; 10];

  for row in 0..grid.height {
    for item in &mut last_seen { *item = None; }
    for col in (0..grid.width).rev() {
      let cell = grid.value(row, col);
      let last_col = (cell as usize..10).filter_map(|idx| last_seen[idx]).min().unwrap_or(grid.width as u8 - 1);
      let distance = last_col - col as u8;

      right_grid.set_value(distance, row, col);
      last_seen[cell as usize] = Some(col as u8);
    }
  }
  right_grid
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_with_sample() {
    let sample_path: std::path::PathBuf = ["input", "day-8", "sample"].iter().collect();
    let input = std::fs::read_to_string(sample_path).expect("Unable to read file");
    assert_eq!(solve(&input), 8);
  }

  #[test]
  fn solve_with_puzzle() {
    let puzzle_path: std::path::PathBuf = ["input", "day-8", "puzzle"].iter().collect();
    let input = std::fs::read_to_string(puzzle_path).expect("Unable to read file");
    assert_eq!(solve(&input), 672280);

  }

  #[test]
  fn initialize_grid_with_values() {
    let grid_str = "2345\n5678\n9012";
    let grid = initialize_grid(grid_str);
    assert_eq!(grid.width, 4);
    assert_eq!(grid.height, 3);
    assert_eq!(grid.height, 3);
    assert_eq!(grid.grid, [2,3,4,5, 5,6,7,8, 9,0,1,2]);
  }

  #[test]
  fn test_solve() {
    let grid_str = "2345\n5678\n9012";
    let result = solve(grid_str);

    assert_eq!(result, 2);
  }

  #[test]
  fn test_top_trees() {
    let grid_str = "2345\n5678\n9012";
    let grid = initialize_grid(grid_str);
    let trees_grid = top_trees(&grid);

    assert_eq!(trees_grid.grid, [0,0,0,0, 1,1,1,1, 2,1,1,1]);
  }

  #[test]
  fn test_bottom_trees() {
    let grid_str = "2345\n5678\n9012";
    let grid = initialize_grid(grid_str);
    let trees_grid = bottom_trees(&grid);

    assert_eq!(trees_grid.grid, [1,1,1,1, 1,1,1,1, 0,0,0,0]);
  }


  #[test]
  fn test_left_trees() {
    let grid_str = "2345\n5678\n9012";
    let grid = initialize_grid(grid_str);
    let trees_grid = left_trees(&grid);

    assert_eq!(trees_grid.grid, [0,1,2,3, 0,1,2,3, 0,1,2,3]);
  }

  #[test]
  fn test_right_trees() {
    let grid_str = "2345\n5678\n9012";
    let grid = initialize_grid(grid_str);
    let trees_grid = right_trees(&grid);

    assert_eq!(trees_grid.grid, [1,1,1,0, 1,1,1,0, 3,1,1,0]);
  }
}
