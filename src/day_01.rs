struct Day01;

impl Day01 {
    pub fn part_1(s: &str) -> i32 {
        s.lines().fold((0, 0), |(sum, max), line| {
            let sum = if line.is_empty() { 0 }
                          else { sum + line.parse::<i32>().unwrap() };
            (sum, max.max(sum))
        }).1
    }

    pub fn part_2(s: &str) -> i32 {
        fn add_sorted(v: &mut Vec<i32>, i: i32) {
            v.push(i);
            v.sort_unstable();
            v.pop();
        }

        let mut max = vec![0,0,0];

        let mut current = 0;
        for line in s.lines() {
            if line.is_empty() {
                add_sorted(&mut max, -current);
                current = 0;
            } else {
                current += line.parse::<i32>().unwrap();
            }
        }
        add_sorted(&mut max, -current);

        -max[0..3].into_iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day01::part_1, "input/day_01_0.txt", 24000);
    }

    #[test]
    fn part_1_1() {
        test(&Day01::part_1, "input/day_01_1.txt", 72478);
    }

    #[test]
    fn part_2_0() {
        test(&Day01::part_2, "input/day_01_0.txt", 45000);
    }

    #[test]
    fn part_2_1() {
        test(&Day01::part_2, "input/day_01_1.txt", 210367);
    }

    fn test(f: &dyn Fn(&str) -> i32, path: &str, r: i32) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}
