use std::{env, fs};

fn main() {
    let path = env::args().nth(1).expect("An input file must be provided");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let result = sum_priorities(&input);

    println!("Result sum priority: {}", result);
}

fn sum_priorities(rucksacks: &str) -> u32 {
    let lines = rucksacks.split('\n');

    lines.map(|l| rucksack_duplicate_priority(l) as u32).sum()
}

fn rucksack_duplicate_priority(line: &str) -> u8 {
    let duplicate = find_duplicate_in_rucksack(line);
    // note: this is slightly inefficient, since find_duplicate gets
    // the priority, then converts it to a char, and then we convert
    // it back here. It should be a pretty quick converstion, and I
    // think the interface is slightly cleaner, so :shrug:
    calculate_priority(duplicate)
}

fn find_duplicate_in_rucksack(rucksack: &str) -> char {
    let (left, right) = get_compartments(rucksack);

    let left_mask = compartment_mask(left);
    let right_mask = compartment_mask(right);

    let shared = left_mask & right_mask;
    let priority = mask_to_priority(shared);

    priority_to_char(priority)
}

fn compartment_mask(compartment: &str) -> u64 {
    let mut mask: u64 = 0;

    for c in compartment.chars() {
        let mask_idx = calculate_priority(c);
        mask |= 1 << mask_idx;
    }

    mask
}

fn mask_to_priority(mask: u64) -> u8 {
    mask.trailing_zeros() as u8
}

fn priority_to_char(priority: u8) -> char {
    if priority < 27 {
        return (priority+96) as char;
    }
    (priority+64-26) as char
}

fn get_compartments(rucksack: &str) -> (&str, &str) {
    let midpoint = rucksack.len() / 2;
    (&rucksack[..midpoint], &rucksack[midpoint..])
}

fn calculate_priority(char: char) -> u8 {
    let c = char as u8;

    if c >= 97 {
        return c-96; // 96 = 'a' ascii value
    }
    c-64+26 // 64 = 'A' ascii value, 26=a-z range
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_priority_sample() {
        assert_eq!(calculate_priority('a'), 1);
        assert_eq!(calculate_priority('z'), 26);
        assert_eq!(calculate_priority('A'), 27);
        assert_eq!(calculate_priority('Z'), 52);

        assert_eq!(calculate_priority('p'), 16);
        assert_eq!(calculate_priority('L'), 38);
    }

    #[test]
    fn get_compartments_sample() {
        assert_eq!(get_compartments("vJrwpWtwJgWrhcsFMMfFFhFp"), ("vJrwpWtwJgWr", "hcsFMMfFFhFp"));

    }

    #[test]
    fn find_duplicate_sample() {
        assert_eq!(find_duplicate_in_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
        assert_eq!(find_duplicate_in_rucksack("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 'L');
        assert_eq!(find_duplicate_in_rucksack("PmmdzqPrVvPwwTWBwg"), 'P');

    }
}
