pub struct RingBuffer {
  internal: Vec<char>,
  capacity: usize,
  head: usize
}

impl RingBuffer {
  pub fn new(size: usize) -> RingBuffer {
    RingBuffer {
      internal: vec![],
      capacity: size,
      head: 0
    }
  }

  pub fn push(&mut self, element: char) -> Option<char> {
    if self.internal.len() < self.capacity {
      self.internal.push(element);
      None
    } else {
      let lost = self.internal[self.head];
      self.internal[self.head] = element;
      self.head = (self.head + 1) % self.capacity;
      Some(lost)
    }
  }

  pub fn len(&self) -> usize {
    self.internal.len()
  }

  pub fn at(&self, idx: usize) -> char {
    self.internal[(idx + self.head) % self.len()]
  }
}

impl<'a> std::iter::IntoIterator for &'a RingBuffer {
  type Item = <std::slice::Iter<'a, char> as Iterator>::Item;
  type IntoIter = std::slice::Iter<'a, char>;

  fn into_iter(self) -> Self::IntoIter {
      self.internal.as_slice().into_iter()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_push_only_fills_to_capacity() {
    let mut buffer = RingBuffer::new(2);

    assert_eq!(buffer.push('A'), None);
    assert_eq!(buffer.len(), 1);

    assert_eq!(buffer.push('B'), None);
    assert_eq!(buffer.len(), 2);

    assert_eq!(buffer.push('C'), Some('A'));
    assert_eq!(buffer.len(), 2);
  }

  #[test]
  fn test_push_removes_from_ring_buffer() {
    let mut buffer = RingBuffer::new(2);

    buffer.push('A');
    assert_eq!(buffer.at(0), 'A');

    buffer.push('B');
    assert_eq!(buffer.at(0), 'A');
    assert_eq!(buffer.at(1), 'B');

    buffer.push('C');
    assert_eq!(buffer.len(), 2);
    assert_eq!(buffer.at(0), 'B');
    assert_eq!(buffer.at(1), 'C');
  }
}