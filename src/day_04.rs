struct Day04;

impl Day04 {
    pub fn part_1(s: &str) -> i32 {
        let mut n = 0;
        for line in s.lines() {
            let mut pairs = line.split(',')
                .map(|s| s.split('-')
                    .map(|a| a.parse::<i32>().unwrap()))
                .flatten();
            let a = (pairs.next().unwrap(), pairs.next().unwrap());
            let b = (pairs.next().unwrap(), pairs.next().unwrap());
            if a.0 <= b.0 && b.1 <= a.1 || b.0 <= a.0 && a.1 <= b.1 {
                n += 1;
            }
        }
        n
    }

    pub fn part_2(s: &str) -> i32 {
        let mut n = 0;
        for line in s.lines() {
            let mut pairs = line.split(',')
                .map(|s| s.split('-')
                    .map(|a| a.parse::<i32>().unwrap()))
                .flatten();
            let a = (pairs.next().unwrap(), pairs.next().unwrap());
            let b = (pairs.next().unwrap(), pairs.next().unwrap());
            if a.0 <= b.0 && b.0 <= a.1 ||
               a.0 <= b.1 && b.1 <= a.1 ||
               b.0 <= a.0 && a.0 <= b.1 ||
               b.0 <= a.1 && a.1 <= b.1 {
                n += 1;
            }
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day04::part_1, "input/day_04_0.txt", 2);
    }

    #[test]
    fn part_1_1() {
        test(&Day04::part_1, "input/day_04_1.txt", 526);
    }

    #[test]
    fn part_2_0() {
        test(&Day04::part_2, "input/day_04_0.txt", 4);
    }

    #[test]
    fn part_2_1() {
        test(&Day04::part_2, "input/day_04_1.txt", 886);
    }

    fn test(f: &dyn Fn(&str) -> i32, path: &str, r: i32) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}
