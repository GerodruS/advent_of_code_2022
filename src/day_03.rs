use std::str::Bytes;

struct Day03;

impl Day03 {
    fn char_to_i(c: u8) -> usize {
        let c = if 'a' as u8 <= c {
            c - 'a' as u8
        } else {
            c - 'A' as u8 + 26
        };
        c as usize
    }

    fn next_char(chars: &mut Bytes) -> usize {
        Self::char_to_i(chars.next().unwrap())
    }

    pub fn part_1(s: &str) -> i32 {
        let mut count = [0_u8;26*2];
        let mut r = 0;
        for line in s.lines() {
            let n = line.len();
            let mut chars = line.bytes();
            for _ in 0..n/2 {
                let c = Self::next_char(&mut chars);
                count[c] |= 1;
            }
            for _ in n/2..n {
                let c = Self::next_char(&mut chars);
                if count[c] == 1 {
                    r += c as i32 + 1;
                    break
                }
            }
            count.fill(0);
        }
        r
    }

    pub fn part_2(s: &str) -> i32 {
        let mut count = [0_u8;26*2];
        let mut r = 0;
        let mut lines = s.lines();
        while let Some(line) = lines.next() {
            for c in line.bytes() {
                let c = Self::char_to_i(c);
                count[c] |= 1;
            }
            for c in lines.next().unwrap().bytes() {
                let c = Self::char_to_i(c);
                count[c] |= 2;
            }
            for c in lines.next().unwrap().bytes() {
                let c = Self::char_to_i(c);
                if count[c] == 3 {
                    r += c as i32 + 1;
                    break
                }
            }
            count.fill(0);
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day03::part_1, "input/day_03_0.txt", 157);
    }

    #[test]
    fn part_1_1() {
        test(&Day03::part_1, "input/day_03_1.txt", 7716);
    }

    #[test]
    fn part_2_0() {
        test(&Day03::part_2, "input/day_03_0.txt", 70);
    }

    #[test]
    fn part_2_1() {
        test(&Day03::part_2, "input/day_03_1.txt", 2973);
    }

    fn test(f: &dyn Fn(&str) -> i32, path: &str, r: i32) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}
