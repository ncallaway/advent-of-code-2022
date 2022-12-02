use std::{env, fs};

fn main() {
  // file path provided by user
  // let input = env::args().nth(1).unwrap();
  let path = env::args().nth(1).expect("An input file must be provided");

  let input = fs::read_to_string(path).expect("Unable to read file");
  let result = max_calories(&input);

  println!("Result total calories: {}", result);
}

fn max_calories(calories: &str) -> u32 {
  // split on newlines
  let elves = parse_elves(calories);

  // let real_result = result.iter().map(|x| x.iter().sum::<u32>()).sum();
  // real_result
  sum_last_elf(elves)
}

fn parse_elves(calories: &str) -> Vec<Vec<u32>> {
  let mut vec = Vec::new();
  let mut inner = Vec::new();
  let lines = calories.split('\n');

  for line in lines {
    if !line.is_empty() {
      inner.push(line.parse::<u32>().expect("A value could not be converted to a u32"))
    } else if !inner.is_empty() {
      vec.push(inner);
      inner = Vec::new();
    }
  }

  if !inner.is_empty() {
    vec.push(inner);
  }

  vec
}

fn sum_last_elf(elves: Vec<Vec<u32>>) -> u32 {
  let total_kcals = elves.iter().map(|x| x.iter().sum::<u32>());

  total_kcals.max().unwrap_or(0)
}

// fn convert_to_vec(split: )

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn max_calories_sums_basic_calories() {
        assert_eq!(max_calories("100\n200"), 300);
    }

    #[test]
    fn max_calories_sums_largest_calories() {
        assert_eq!(max_calories("100\n200\n\n50\n100"), 300);
    }
}
