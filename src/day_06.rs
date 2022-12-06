struct Day06;

impl Day06 {
    pub fn solve(s: &str, c: usize) -> i32 {
        for (i, w) in s.as_bytes().windows(c).enumerate() {
            let mut unique = true;
            for j in 0..c - 1 {
                for k in j + 1..c {
                    unique = unique && w[j] != w[k];
                }
            }
            if unique { return (i + c) as i32 }
        }
        -1
    }

    pub fn part_1(s: &str) -> i32 {
        Self::solve(s, 4)
    }

    pub fn part_2(s: &str) -> i32 {
        Self::solve(s, 14)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day06::part_1, "mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7);
        test(&Day06::part_1, "bvwbjplbgvbhsrlpgdmjqwftvncz", 5);
        test(&Day06::part_1, "nppdvjthqldpwncqszvftbrmjlhg", 6);
        test(&Day06::part_1, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10);
        test(&Day06::part_1, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11);
    }

    #[test]
    fn part_1_1() {
        test_file(&Day06::part_1, "input/day_06_1.txt", 1134);
    }

    #[test]
    fn part_2_0() {
        test(&Day06::part_2, "mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19);
        test(&Day06::part_2, "bvwbjplbgvbhsrlpgdmjqwftvncz", 23);
        test(&Day06::part_2, "nppdvjthqldpwncqszvftbrmjlhg", 23);
        test(&Day06::part_2, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29);
        test(&Day06::part_2, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26);
    }

    #[test]
    fn part_2_1() {
        test_file(&Day06::part_2, "input/day_06_1.txt", 2263);
    }

    fn test(f: &dyn Fn(&str) -> i32, input: &str, r: i32) {
        assert_eq!(f(input), r);
    }

    fn test_file(f: &dyn Fn(&str) -> i32, path: &str, r: i32) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}
