pub struct CrateStack {
  internal: Vec<char>,
}


impl CrateStack {
  pub fn new() -> CrateStack {
    CrateStack {
      internal: vec![],
    }
  }

  pub fn push(&mut self, element: char) {
    self.internal.push(element);
  }

  pub fn _len(&self) -> usize{
    self.internal.len()
  }

  pub fn reverse(&mut self) {
    self.internal.reverse();
  }

  pub fn pop(&mut self) -> Option<char> {
    match !self.is_empty() {
      true => Some(self.internal.remove(self.internal.len() - 1)),
      false => None
    }

  }

  pub fn peek(&self) -> Option<char> {
    match !self.is_empty() {
      true => Some(self.internal[self.internal.len() - 1]),
      false => None
    }
  }

  pub fn is_empty(&self) -> bool {
    self.internal.is_empty()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn push_pop() {
    let mut stack = CrateStack::new();

    assert!(stack.is_empty());
    stack.push('A');
    assert!(!stack.is_empty());
    let result = stack.pop();
    assert_eq!(result, Some('A'));
    assert!(stack.is_empty());
  }

  #[test]
  fn push_reverse_peek() {
    let mut stack = CrateStack::new();

    assert_eq!(stack.peek(), None);
    stack.push('A');
    stack.push('B');
    stack.push('C');
    assert_eq!(stack.peek(), Some('C'));
    stack.reverse();
    assert_eq!(stack.peek(), Some('A'));
  }

  #[test]
  fn pop_empty() {
    let mut stack = CrateStack::new();

    assert_eq!(stack.pop(), None);
    stack.push('A');
    assert_eq!(stack.pop(), Some('A'));
    assert_eq!(stack.pop(), None);
  }
}
