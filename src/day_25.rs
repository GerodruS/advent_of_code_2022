struct Day25;

impl Day25 {
    pub fn s_to_i(s: &str) -> i64 {
        let mut i = 0;
        for &ch in s.as_bytes() {
            i = i * 5 + match ch {
                b'=' => -2,
                b'-' => -1,
                ch => (ch - b'0') as i64,
            };
        }
        i
    }

    pub fn i_to_s(mut i: i64) -> String {
        let mut r = String::new();
        while 0 < i {
            let n = i % 5;
            i = i / 5;
            let ch = if n < 3 {
                (b'0' + n as u8) as char
            } else {
                i += 1;
                match n {
                    3 => '=',
                    4 => '-',
                    _ => panic!("n={n}"),
                }
            };
            r.push(ch);
        }
        r.chars().rev().collect()
    }

    pub fn part_1(s: &str) -> String {
        let mut sum = 0;
        for line in s.lines() {
            sum += Self::s_to_i(line);
        }
        Self::i_to_s(sum)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_0_0() {
        assert_eq!(1, Day25::s_to_i("1"));
        assert_eq!(2, Day25::s_to_i("2"));
        assert_eq!(3, Day25::s_to_i("1="));
        assert_eq!(4, Day25::s_to_i("1-"));
        assert_eq!(5, Day25::s_to_i("10"));
        assert_eq!(6, Day25::s_to_i("11"));
        assert_eq!(7, Day25::s_to_i("12"));
        assert_eq!(8, Day25::s_to_i("2="));
        assert_eq!(9, Day25::s_to_i("2-"));
        assert_eq!(10, Day25::s_to_i("20"));
        assert_eq!(15, Day25::s_to_i("1=0"));
        assert_eq!(20, Day25::s_to_i("1-0"));
        assert_eq!(2022, Day25::s_to_i("1=11-2"));
        assert_eq!(12345, Day25::s_to_i("1-0---0"));
        assert_eq!(314159265, Day25::s_to_i("1121-1110-1=0"));

        assert_eq!(Day25::i_to_s(1), "1");
        assert_eq!(Day25::i_to_s(2), "2");
        assert_eq!(Day25::i_to_s(3), "1=");
        assert_eq!(Day25::i_to_s(4), "1-");
        assert_eq!(Day25::i_to_s(5), "10");
        assert_eq!(Day25::i_to_s(6), "11");
        assert_eq!(Day25::i_to_s(7), "12");
        assert_eq!(Day25::i_to_s(8), "2=");
        assert_eq!(Day25::i_to_s(9), "2-");
        assert_eq!(Day25::i_to_s(10), "20");
        assert_eq!(Day25::i_to_s(15), "1=0");
        assert_eq!(Day25::i_to_s(20), "1-0");
        assert_eq!(Day25::i_to_s(2022), "1=11-2");
        assert_eq!(Day25::i_to_s(12345), "1-0---0");
        assert_eq!(Day25::i_to_s(314159265), "1121-1110-1=0");
    }

    #[test]
    fn part_1_0() {
        test(&Day25::part_1, "input/day_25_0.txt", "2=-1=0");
    }

    #[test]
    fn part_1_1() {
        test(&Day25::part_1, "input/day_25_1.txt", "2-02===-21---2002==0");
    }

    fn test(f: &dyn Fn(&str) -> String, path: &str, r: &str) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}