pub fn solve(input: &str) -> u32 {
  calculate_score(input)
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Shape {
    Rock = 0,
    Paper,
    Scissors,
}

impl Shape {
    fn from_i32(value: i32) -> Shape {
        match value {
            0 => Shape::Rock,
            1 => Shape::Paper,
            2 => Shape::Scissors,
            _ => panic!("Unknown Shape: {}", value),
        }
    }
}


fn calculate_score(strategy: &str) -> u32 {
    let lines = strategy.split('\n');

    lines.map(score_line).sum()
}

fn score_line(line: &str) -> u32 {
    let (theirs, mine) = convert_line(line);

    score_win(theirs, mine) + score_shape(mine)
}

fn convert_line(line: &str) -> (Shape, Shape) {
    let shapes: Vec<&str> = line.split(' ').take(2).collect();

    let theirs = convert(shapes[0]);
    let mine = calculate_my_shape(theirs, shapes[1]);

    (theirs, mine)
}

fn convert(encoded_shape: &str) -> Shape {
    match encoded_shape {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("unrecognized encoded shape")
    }
}

fn calculate_my_shape(theirs: Shape, encoded_result: &str) -> Shape {
    match encoded_result {

        // lose
        "X" => Shape::from_i32(((theirs as i32) - 1).rem_euclid(3)),

        // draw
        "Y" => theirs,

        // win
        "Z" => Shape::from_i32(((theirs as i32) + 1).rem_euclid(3)),

        _ => panic!("unrecognized encoded result")
    }
}

fn score_win(theirs: Shape, mine: Shape) -> u32 {
    if theirs == mine { return 3 }

    if mine as u8 == theirs as u8 + 1 || mine == Shape::Rock && theirs == Shape::Scissors {
        return 6;
    }

    0
}

fn score_shape(mine: Shape) -> u32 {
    match mine {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_score_calculates_basic_score() {
        assert_eq!(calculate_score("A Y"), 4);
    }

    #[test]
    fn calculate_score_calculates_multiple_scores() {
        assert_eq!(calculate_score("A Y\nB X"), 5);
    }

    #[test]
    fn score_line_calculates_basic_score() {
        assert_eq!(score_line("B X"), 1);
    }

    #[test]
    fn test_score_shape() {
        assert_eq!(score_shape(Shape::Rock), 1);
        assert_eq!(score_shape(Shape::Paper), 2);
        assert_eq!(score_shape(Shape::Scissors), 3);
    }

    #[test]
    fn score_win_draw() {
        assert_eq!(score_win(Shape::Rock, Shape::Rock), 3);
    }

    #[test]
    fn score_win_win() {
        assert_eq!(score_win(Shape::Rock, Shape::Paper), 6);
        assert_eq!(score_win(Shape::Paper, Shape::Scissors), 6);
        assert_eq!(score_win(Shape::Scissors, Shape::Rock), 6);
    }

    #[test]
    fn score_win_loss() {
        assert_eq!(score_win(Shape::Rock, Shape::Scissors), 0);
        assert_eq!(score_win(Shape::Paper, Shape::Rock), 0);
        assert_eq!(score_win(Shape::Scissors, Shape::Paper), 0);
    }

    #[test]
    fn test_convert() {
        assert_eq!(convert("A"), Shape::Rock);
        assert_eq!(convert("B"), Shape::Paper);
        assert_eq!(convert("C"), Shape::Scissors);
    }

    #[test]
    fn calculate_my_shape_draw() {
        assert_eq!(calculate_my_shape(Shape::Rock, "Y"), Shape::Rock);
        assert_eq!(calculate_my_shape(Shape::Paper, "Y"), Shape::Paper);
        assert_eq!(calculate_my_shape(Shape::Scissors, "Y"), Shape::Scissors);
    }

    #[test]
    fn calculate_my_shape_loss() {
        assert_eq!(calculate_my_shape(Shape::Rock, "X"), Shape::Scissors);
        assert_eq!(calculate_my_shape(Shape::Paper, "X"), Shape::Rock);
        assert_eq!(calculate_my_shape(Shape::Scissors, "X"), Shape::Paper);
    }

    #[test]
    fn calculate_my_shape_win() {
        assert_eq!(calculate_my_shape(Shape::Rock, "Z"), Shape::Paper);
        assert_eq!(calculate_my_shape(Shape::Paper, "Z"), Shape::Scissors);
        assert_eq!(calculate_my_shape(Shape::Scissors, "Z"), Shape::Rock);
    }

    #[test]
    fn test_convert_line() {
        assert_eq!(convert_line("C Z"), (Shape::Scissors, Shape::Rock));
    }
}
