#!/bin/bash

if [[ -z $1 ]] ; then echo 'no day given'; exit 1; fi

APP_ROOT="$(dirname "$(dirname "$(readlink -fm "$0")")")"

DAY="day-$1"
DAY_UNDER="day_$1"

cd $APP_ROOT

if [[ -d "src/puzzles/$DAY_UNDER" ]] ; then echo "puzzle for $DAY already exists"; exit 1; fi

echo "preparing puzzle for $DAY"
mkdir -p input/$DAY
mkdir -p src/puzzles/$DAY_UNDER

touch input/$DAY/sample
touch input/$DAY/puzzle

echo -e "pub mod puzzle_1;\npub mod puzzle_2;\n" > src/puzzles/$DAY_UNDER/mod.rs
cat <<EOT > src/puzzles/$DAY_UNDER/puzzle_1.rs
pub fn solve(_input: &str) -> u64 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_with_sample() {
    let sample_path: std::path::PathBuf = ["input", "$DAY", "sample"].iter().collect();
    let input = std::fs::read_to_string(sample_path).expect("Unable to read file");
    assert_eq!(solve(&input), 0);
  }

  #[test]
  fn solve_with_puzzle() {
    let puzzle_path: std::path::PathBuf = ["input", "$DAY", "puzzle"].iter().collect();
    let input = std::fs::read_to_string(puzzle_path).expect("Unable to read file");
    assert_eq!(solve(&input), 0);
  }
}
EOT

cp src/puzzles/$DAY_UNDER/puzzle_1.rs src/puzzles/$DAY_UNDER/puzzle_2.rs

echo -e "pub mod $DAY_UNDER;" >> src/puzzles/mod.rs

sed -i "/\_ => None/i \    \(\"$DAY\", \"puzzle-1\"\) => Some\(puzzles::$DAY_UNDER::puzzle_1::solve\)," src/main.rs
sed -i "/\_ => None/i \    \(\"$DAY\", \"puzzle-2\"\) => Some\(puzzles::$DAY_UNDER::puzzle_2::solve\)," src/main.rs

sed -i "/\_ => \"Solution\"/i \    \(\"$DAY\",\) => \"Total XXX\"," src/main.rs

echo -e "\n### Day $1\n\n\`\`\`sh\n#puzzle-1\n\n#puzzle-2\n\n\`\`\`\n" >> README.md