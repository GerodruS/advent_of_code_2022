struct Day20;

impl Day20 {
    pub fn solve(s: &str, decryption_key: i64, repeat: usize) -> i64 {
        let mut numbers: Vec<(usize, i64)> = s.lines().map(
            |line| line.parse::<i64>().unwrap() * decryption_key
        ).enumerate().collect();
        let count = numbers.len();

        for _ in 0..repeat {
            for i in 0..count {
                let i = numbers.iter().position(|&(index, _)| index == i).unwrap();
                let elem = numbers[i];
                numbers.remove(i);
                let i = (i as i64 + elem.1).rem_euclid(count as i64 - 1) as usize;
                numbers.insert(i, elem);
            }
        }

        let zero = numbers.iter().position(|&(_, n)| n == 0).unwrap();
        let a = numbers[(zero + 1000) % count].1;
        let b = numbers[(zero + 2000) % count].1;
        let c = numbers[(zero + 3000) % count].1;
        a + b + c
    }

    pub fn part_1(s: &str) -> i64 {
        Self::solve(s, 1, 1)
    }

    pub fn part_2(s: &str) -> i64 {
        Self::solve(s, 811589153, 10)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day20::part_1, "input/day_20_0.txt", 3);
    }

    #[test]
    fn part_1_1() {
        test(&Day20::part_1, "input/day_20_1.txt", 1087);
    }

    #[test]
    fn part_2_0() {
        test(&Day20::part_2, "input/day_20_0.txt", 1623178306);
    }

    #[test]
    fn part_2_1() {
        test(&Day20::part_2, "input/day_20_1.txt", 13084440324666);
    }

    fn test(f: &dyn Fn(&str) -> i64, path: &str, r: i64) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}