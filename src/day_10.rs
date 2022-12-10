struct Day10;

impl Day10 {
    fn parse_line(line: &str) -> (usize, i32) {
        if line.starts_with("a") {
            (2, line[5..].parse::<i32>().unwrap())
        } else {
            (1, 0)
        }
    }

    pub fn part_1(s: &str) -> i32 {
        let mut signal = 1;
        let mut cycle = 1;
        let mut r = 0;
        for (n, s) in s.lines().map(Self::parse_line) {
            for _ in 0..n - 1 {
                cycle += 1;
                if (cycle - 20) % 40 == 0 {
                    r += cycle * signal;
                }
            }
            signal += s;
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                r += cycle * signal;
            }
        }
        r
    }

    pub fn part_2(s: &str) -> String {
        fn get_char(cycle: i32, signal: i32) -> char {
            if signal - 1 <= cycle % 40 && cycle % 40 <= signal + 1 {
                '#'
            } else {
                '.'
            }
        }

        let mut signal = 1;
        let mut cycle = 0;
        let mut pixels = Vec::with_capacity(247);

        pixels.push('#');
        for (n, s) in s.lines().map(Self::parse_line) {
            for _ in 0..n - 1 {
                cycle += 1;
                if cycle % 40 == 0 { pixels.push('\n'); }
                pixels.push(get_char(cycle, signal));
            }

            signal += s;
            cycle += 1;
            if cycle % 40 == 0 { pixels.push('\n'); }
            pixels.push(get_char(cycle, signal));
        }

        pixels[..240+5].into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day10::part_1, "input/day_10_0.txt", 13140);
    }

    #[test]
    fn part_1_1() {
        test(&Day10::part_1, "input/day_10_1.txt", 17840);
    }

    #[test]
    fn part_2_0() {
        test_file(&Day10::part_2, "input/day_10_0.txt", "input/day_10_0_r.txt");
    }

    #[test]
    fn part_2_1() {
        test_file(&Day10::part_2, "input/day_10_1.txt", "input/day_10_1_r.txt");
    }

    fn test(f: &dyn Fn(&str) -> i32, path: &str, r: i32) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }

    fn test_file(f: &dyn Fn(&str) -> String, path_input: &str, path_result: &str) {
        let input = fs::read_to_string(path_input).unwrap();
        let result = fs::read_to_string(path_result).unwrap();
        assert_eq!(f(&input), result);
    }
}
