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

# PROJECT="$DAY-$PUZZLE"
# cargo new $PROJECT
# mkdir -p "$PROJECT/input"
touch input/$DAY/sample
touch input/$DAY/puzzle

echo -e "pub mod puzzle_1;\npub mod puzzle_2;\n" > src/puzzles/$DAY_UNDER/mod.rs
echo -e "pub fn solve(input: &str) -> u32 {\n  0\n}\n" > src/puzzles/$DAY_UNDER/puzzle_1.rs
cp src/puzzles/$DAY_UNDER/puzzle_1.rs src/puzzles/$DAY_UNDER/puzzle_2.rs

echo -e "pub mod $DAY_UNDER;" >> src/puzzles/mod.rs

sed -i "/\_ => None/i \    \(\"$DAY\", \"puzzle-1\"\) => Some\(puzzles::$DAY_UNDER::puzzle_1::solve\)," src/main.rs
sed -i "/\_ => None/i \    \(\"$DAY\", \"puzzle-2\"\) => Some\(puzzles::$DAY_UNDER::puzzle_2::solve\)," src/main.rs

sed -i "/\_ => \"Solution\"/i \    \(\"$DAY\",\) => \"Total XXX\"," src/main.rs

echo -e "\n## Day $1, Puzzle 1\n\n\`\`\`sh\n\n\`\`\`\n\n## Day $1, Puzzle 1\n\n\`\`\`sh\n\n\`\`\`" >> README.md