struct Day02;

impl Day02 {
    pub fn part_1(s: &str) -> i32 {
        s.lines().fold(0, |score, line|
            score + match line {
                // won:
                "C X" => 6 + 1,
                "A Y" => 6 + 2,
                "B Z" => 6 + 3,
                // draw:
                "A X" => 3 + 1,
                "B Y" => 3 + 2,
                "C Z" => 3 + 3,
                // lost:
                "B X" => 0 + 1,
                "C Y" => 0 + 2,
                "A Z" => 0 + 3,

                _ => panic!("line='{line}'"),
            }
        )
    }

    pub fn part_2(s: &str) -> i32 {
        s.lines().fold(0, |score, line|
            score + match line {
                //
                "A Y" => 1 + 3,
                "B X" => 1 + 0,
                "C Z" => 1 + 6,
                //
                "A Z" => 2 + 6,
                "B Y" => 2 + 3,
                "C X" => 2 + 0,
                //
                "A X" => 3 + 0,
                "B Z" => 3 + 6,
                "C Y" => 3 + 3,

                _ => panic!("line='{line}'"),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day02::part_1, "input/day_02_0.txt", 15);
    }

    #[test]
    fn part_1_1() {
        test(&Day02::part_1, "input/day_02_1.txt", 13924);
    }

    #[test]
    fn part_2_0() {
        test(&Day02::part_2, "input/day_02_0.txt", 12);
    }

    #[test]
    fn part_2_1() {
        test(&Day02::part_2, "input/day_02_1.txt", 13448);
    }

    fn test(f: &dyn Fn(&str) -> i32, path: &str, r: i32) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}