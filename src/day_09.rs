use std::collections::HashSet;

struct Day09;

impl Day09 {
    fn step(h: (i32, i32), t: (i32, i32)) -> (i32, i32) {
        let mut new_t = t;
        let d = (h.0 - t.0, h.1 - t.1);
        if 1 < d.0.abs() || 1 < d.1.abs() {
            new_t.0 += d.0.signum();
            new_t.1 += d.1.signum();
        }
        new_t
    }

    fn parse_line(line: &str) -> ((i32, i32), i32) {
        let dir = match line.as_bytes()[0] {
            b'R' => { (1,0) },
            b'L' => { (-1,0) },
            b'U' => { (0,1) },
            _ => { (0,-1) },
        };
        let n = line[2..line.len()].parse::<i32>().unwrap();
        (dir, n)
    }

    pub fn part_1(s: &str) -> usize {
        let mut h = (0,0);
        let mut t = (0,0);

        let mut set = HashSet::new();
        set.insert(t);

        for (dir, n) in s.lines().map(Self::parse_line) {
            for _ in 0..n {
                h = (h.0 + dir.0, h.1 + dir.1);
                t = Self::step(h, t);
                set.insert(t);
            }
        }
        set.len()
    }

    pub fn part_2(s: &str) -> usize {
        const N: usize = 10;
        let mut r = [(0, 0); N];

        let mut set = HashSet::new();
        set.insert((0,0));

        for (dir, n) in s.lines().map(Self::parse_line) {
            for _ in 0..n {
                r[0] = (r[0].0 + dir.0, r[0].1 + dir.1);
                for i in 1..N {
                    r[i] = Self::step(r[i - 1], r[i]);
                }
                set.insert(r[N - 1]);
            }
        }
        set.len()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day09::part_1, "input/day_09_0.txt", 13);
    }

    #[test]
    fn part_1_1() {
        test(&Day09::part_1, "input/day_09_1.txt", 5858);
    }

    #[test]
    fn part_2_0() {
        test(&Day09::part_2, "input/day_09_2.txt", 36);
    }

    #[test]
    fn part_2_1() {
        test(&Day09::part_2, "input/day_09_1.txt", 2602);
    }

    fn test(f: &dyn Fn(&str) -> usize, path: &str, r: usize) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}
