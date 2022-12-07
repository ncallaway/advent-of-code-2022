use itertools::Itertools;

pub fn solve(input: &str) -> u64 {
  max_calories(input).into()
}

fn max_calories(calories: &str) -> u32 {
  sum_largest_elves(parse_elves(calories), 3)
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

fn sum_largest_elves(elves: Vec<Vec<u32>>, n: usize) -> u32 {
  let kcal_per_elf = elves.iter().map(|x| x.iter().sum::<u32>());
  let largest_elves = kcal_per_elf.sorted_by(|a, b| b.cmp(a)).take(n);
  largest_elves.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_calories_sums_basic_calories() {
        assert_eq!(max_calories("100\n200"), 300);
    }

    #[test]
    fn max_calories_sums_largest_3() {
        assert_eq!(max_calories("100\n200\n\n50\n\n100\n\n200\n200"), 800);
    }

    #[test]
    fn sum_largest_elf_sums_largest() {
        assert_eq!(sum_largest_elves(vec![vec![1], vec![2], vec![3], vec![4]], 2), 7);
    }
}
