struct Solution;

impl Solution {
    pub fn solve(s: &str) -> i32 {
        let mut max = 0;

        let mut current = 0;
        for line in s.lines() {
            if line.is_empty() {
                max = max.max(current);
                current = 0;
            } else {
                current += line.parse::<i32>().unwrap();
            }
        }

        max.max(current)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_0() {
        test("input/day_01_0.txt", 24000);
    }

    #[test]
    fn test_1() {
        test("input/day_01_1.txt", 72478);
    }

    fn test(file_path: &str, r: i32) {
        let input = fs::read_to_string(file_path).unwrap();
        assert_eq!(Solution::solve(&input), r);
    }
}
