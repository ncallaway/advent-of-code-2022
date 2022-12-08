pub fn solve(input: &str) -> u64 {
  let mut grid = initialize_grid(input);
  walk_top_left(&mut grid);
  walk_bottom_right(&mut grid);
  find_visible_count(&grid) as u64
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
  grid: Vec<u32>
}

impl Grid {
  fn index(&self, row: usize, col: usize) -> usize {
    self.width*row + col
  }

  fn value(&self, row: usize, col: usize) -> u32 {
    self.grid[self.index(row, col)]
  }

  fn set_value(&mut self, value: u32, row: usize, col: usize) {
    let idx = self.index(row, col);
    self.grid[idx] = value;
  }
}

fn initialize_grid(input: &str) -> Grid {
  let mut width = 0;
  let mut height = 0;
  let mut grid: Vec<u32> = Vec::new();

  for line in input.split('\n') {
    if width == 0 { width = line.len(); }
    height += 1;

    for c in line.chars() {
      let value: u32 = (c as u8 - 48).into(); // 48 is '0'in ascii
      grid.push(value);
    }
  }

  Grid { width, height, grid }
}

fn find_visible_count(grid: &Grid) -> usize {
  // auto-count the edge nodes
  let mut count = grid.width*2 + grid.height*2 - 4;

  for col in 1..grid.width-1 {
    for row in 1..grid.height-1 {
      // let idx = grid.index(row, col);
      let cell = grid.value(row, col);
      let value = read_value(cell, Register::VALUE);
      let l = read_value(cell, Register::LEFT);
      let r = read_value(cell, Register::RIGHT);
      let t = read_value(cell, Register::TOP);
      let b = read_value(cell, Register::BOTTOM);

      if value > l || value > r || value > t || value > b {
        count += 1;
      }
    }
  }

  count
}

fn walk_top_left(grid: &mut Grid) {
  for col in 1..grid.width-1 {
    for row in 1..grid.height-1 {
      // let idx = grid.index(row, col);
      let mut cell = grid.value(row, col);
      // let top_idx = grid.index(row, col-1);
      // let left_idx = grid.index(row-1, col);

      let left = grid.value(row, col-1);
      let top = grid.value(row-1, col);

      let cell_left = std::cmp::max(read_value(left, Register::LEFT), read_value(left, Register::VALUE));
      let cell_top = std::cmp::max(read_value(top, Register::TOP), read_value(top, Register::VALUE));

      cell = write_value(cell, cell_left, Register::LEFT);
      cell = write_value(cell, cell_top, Register::TOP);

      grid.set_value(cell, row, col);
    }
  }
}

fn walk_bottom_right(grid: &mut Grid) {
  for col in (1..grid.width-1).rev() {
    for row in (1..grid.height-1).rev() {
      // let idx = grid.index(row, col);
      let mut cell = grid.value(row, col);
      // let top_idx = grid.index(row, col-1);
      // let left_idx = grid.index(row-1, col);

      let right = grid.value(row, col+1);
      let bottom = grid.value(row+1, col);

      let cell_right = std::cmp::max(read_value(right, Register::RIGHT), read_value(right, Register::VALUE));
      let cell_bottom = std::cmp::max(read_value(bottom, Register::BOTTOM), read_value(bottom, Register::VALUE));

      cell = write_value(cell, cell_right, Register::RIGHT);
      cell = write_value(cell, cell_bottom, Register::BOTTOM);

      grid.set_value(cell, row, col);
    }
  }
}


enum Register {
  VALUE = 0,
  LEFT = 1,
  TOP = 2,
  RIGHT = 3,
  BOTTOM = 4,
}



fn write_value(original: u32, value: u8, register: Register) -> u32 {
  debug_assert!(value < 16);
  let r = register as u8;

  let mask: u32 = 0b111 << (r*4);
  let bits: u32 = (value as u32 & 0b1111) << (r*4);

  original & (!mask) | bits
}

fn read_value(original: u32, register: Register) -> u8 {
  let r: u8 = register as u8;
  ((original >> (r*4)) & 0b1111) as u8
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_with_sample() {
    let sample_path: std::path::PathBuf = ["input", "day-8", "sample"].iter().collect();
    let input = std::fs::read_to_string(sample_path).expect("Unable to read file");
    assert_eq!(solve(&input), 21);
  }

  #[test]
  fn solve_with_puzzle() {
    let puzzle_path: std::path::PathBuf = ["input", "day-8", "puzzle"].iter().collect();
    let input = std::fs::read_to_string(puzzle_path).expect("Unable to read file");
    assert_eq!(solve(&input), 1698);

  }

  #[test]
  fn initialize_grid_with_values() {
    let grid_str = "2345\n5678\n9012";
    let grid = initialize_grid(grid_str);
    assert_eq!(grid.width, 4);
    assert_eq!(grid.height, 3);
    assert_eq!(grid.height, 3);
    assert_eq!(grid.grid, [2,3,4,5,5,6,7,8,9,0,1,2]);
  }

  #[test]
  fn test_walk_top_left() {
    let grid_str = "2345\n5678\n9012";
    let mut grid = initialize_grid(grid_str);
    walk_top_left(&mut grid);

    assert_eq!(read_value(grid.value(1, 1), Register::VALUE), 6);
    assert_eq!(read_value(grid.value(1, 1), Register::LEFT), 5);
    assert_eq!(read_value(grid.value(1, 1), Register::TOP), 3);

    assert_eq!(read_value(grid.value(1, 2), Register::VALUE), 7);
    assert_eq!(read_value(grid.value(1, 2), Register::LEFT), 6);
    assert_eq!(read_value(grid.value(1, 2), Register::TOP), 4);
  }

  #[test]
  fn test_walk_bottom_right() {
    let grid_str = "2345\n5678\n9012";
    let mut grid = initialize_grid(grid_str);
    walk_bottom_right(&mut grid);

    assert_eq!(read_value(grid.value(1, 1), Register::VALUE), 6);
    assert_eq!(read_value(grid.value(1, 1), Register::BOTTOM), 0);
    assert_eq!(read_value(grid.value(1, 1), Register::RIGHT), 8);

    assert_eq!(read_value(grid.value(1, 2), Register::VALUE), 7);
    assert_eq!(read_value(grid.value(1, 2), Register::BOTTOM), 1);
    assert_eq!(read_value(grid.value(1, 2), Register::RIGHT), 8);
  }

  #[test]
  fn test_find_visible_count() {
    let grid_str = "2345\n5678\n9012";
    let mut grid = initialize_grid(grid_str);
    walk_top_left(&mut grid);
    walk_bottom_right(&mut grid);

    assert_eq!(find_visible_count(&grid), 12);
  }


  #[test]
  fn write_values() {
    assert_eq!(write_value(0, 5, Register::VALUE), 5);
    assert_eq!(write_value(0, 5, Register::LEFT), 0b0101_0000);
    assert_eq!(write_value(0, 5, Register::TOP), 0b0101_0000_0000);

    let mut value = write_value(0, 5, Register::VALUE);
    value = write_value(value, 3, Register::LEFT);
    value = write_value(value, 7, Register::TOP);
    assert_eq!(value, 0b0111_0011_0101);
  }

  #[test]
  fn read_values() {
    assert_eq!(read_value(0b0101, Register::VALUE), 5);
    assert_eq!(read_value(0b0101_0000, Register::LEFT), 5);
    assert_eq!(read_value(0b0101_0000_0000, Register::TOP), 5);

    assert_eq!(read_value(0b0111_0011_0101, Register::VALUE), 5);
    assert_eq!(read_value(0b0111_0011_0101, Register::LEFT), 3);
    assert_eq!(read_value(0b0111_0011_0101, Register::TOP), 7);
    assert_eq!(read_value(0b0110_0111_0011_0101, Register::RIGHT), 6);
    assert_eq!(read_value(0b0010_0110_0111_0011_0101, Register::BOTTOM), 2);
  }
}
