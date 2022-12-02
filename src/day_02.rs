struct Solution;

impl Solution {
    pub fn part_1(s: &str) -> i32 {
        const L_ROCK: char = 'A';
        const L_PAPER: char = 'B';
        const L_SCISSORS: char = 'C';

        const R_ROCK: char = 'X';
        const R_PAPER: char = 'Y';
        const R_SCISSORS: char = 'Z';

        let mut score = 0;

        for line in s.lines() {
            let (left, right) = {
                let mut chars = line.chars();
                let left = chars.next().unwrap();
                chars.next();
                let right = chars.next().unwrap();
                (left, right)
            };

            score += match right {
                R_ROCK => 1,
                R_PAPER => 2,
                R_SCISSORS => 3,
                _ => panic!("right={right}"),
            };

            score += match (left, right) {
                (L_ROCK, R_PAPER) | (L_PAPER, R_SCISSORS) | (L_SCISSORS, R_ROCK) => 6,
                (L_ROCK, R_ROCK) | (L_PAPER, R_PAPER) | (L_SCISSORS, R_SCISSORS) => 3,
                (L_ROCK, R_SCISSORS) | (L_PAPER, R_ROCK) | (L_SCISSORS, R_PAPER) => 0,
                _ => panic!("left={left} right={right}"),
            };
        }

        score
    }

    pub fn part_2(s: &str) -> i32 {
        const L_ROCK: char = 'A';
        const L_PAPER: char = 'B';
        const L_SCISSORS: char = 'C';

        const R_LOSE: char = 'X';
        const R_DRAW: char = 'Y';
        const R_WIN: char = 'Z';

        let mut score = 0;

        for line in s.lines() {
            let (left, right) = {
                let mut chars = line.chars();
                let left = chars.next().unwrap();
                chars.next();
                let right = chars.next().unwrap();
                (left, right)
            };

            score += match right {
                R_WIN => 6,
                R_DRAW => 3,
                R_LOSE => 0,
                _ => panic!("right={right}"),
            };

            score += match (left, right) {
                (L_ROCK, R_DRAW) | (L_PAPER, R_LOSE) | (L_SCISSORS, R_WIN) => 1,
                (L_ROCK, R_WIN) | (L_PAPER, R_DRAW) | (L_SCISSORS, R_LOSE) => 2,
                (L_ROCK, R_LOSE) | (L_PAPER, R_WIN) | (L_SCISSORS, R_DRAW) => 3,
                _ => panic!("left={left} right={right}"),
            };
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Solution::part_1, "input/day_02_0.txt", 15);
    }

    #[test]
    fn part_1_1() {
        test(&Solution::part_1, "input/day_02_1.txt", 13924);
    }

    #[test]
    fn part_2_0() {
        test(&Solution::part_2, "input/day_02_0.txt", 12);
    }

    #[test]
    fn part_2_1() {
        test(&Solution::part_2, "input/day_02_1.txt", 13448);
    }

    fn test(f: &dyn Fn(&str) -> i32, path: &str, r: i32) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}
