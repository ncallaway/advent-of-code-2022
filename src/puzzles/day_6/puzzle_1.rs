use super::buffer::RingBuffer;

pub fn solve(input: &str) -> u32 {
  let marker_size = 4;

  let mut buffer = RingBuffer::new(marker_size);
  for (idx, c) in input.chars().enumerate() {
    buffer.push(c);
    let mut mask = 0;
    for c in &buffer {
      mask = add_to_mask(mask, *c);
    }

    if buffer.len() >= marker_size && mask_count(mask) >= marker_size as u32 {
      return idx as u32 + 1
    }
  }

  0
}

fn add_to_mask(mut mask: u32, c: char) -> u32 {
  mask |= 1 << get_mask_index(c);
  mask
}

fn mask_count(mask: u32) -> u32 {
  mask.count_ones()
}


fn get_mask_index(char: char) -> u8 {
  char as u8 - 97 // 97 = 'a' in ascii
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_test() {
    assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);

  }

  #[test]
  fn mask_basics() {
    let mut mask = 0;
    assert_eq!(mask_count(mask), 0);

    mask = add_to_mask(mask, 'a');
    assert_eq!(mask, 1);
    assert_eq!(mask_count(mask), 1);

    mask = add_to_mask(mask, 'b');
    assert_eq!(mask, 3);
    assert_eq!(mask_count(mask), 2);

    mask = add_to_mask(mask, 'b');
    assert_eq!(mask_count(mask), 2);
  }
}


