use std::cmp::Ordering;

struct Day13;

#[derive(Clone, Debug)]
enum Element {
    Int(i32),
    List(Vec<Element>),
}

impl Day13 {
    fn get_closing_position(s: &[u8]) -> Option<usize> {
        let mut n = 0;
        for (i, &c) in s.iter().enumerate() {
            match c {
                b'[' => { n += 1; }
                b']' => { n -= 1; }
                _ => {}
            }
            if n == 0 { return Some(i) }
        }
        None
    }

    fn get_element(s: &[u8]) -> (Element, &[u8]) {
        if let Some(&c) = s.first() {
            if (c as char).is_digit(10) {
                let end = s.iter().position(|&c| c == b',').unwrap_or(s.len());
                let new_s = if end < s.len() {
                    &s[end + 1..]
                } else {
                    &s[end - 1..end - 1]
                };
                return (Element::Int(std::str::from_utf8(&s[0..end]).unwrap().parse::<i32>().unwrap()), new_s)
            } else if c == b'[' {
                let end = Self::get_closing_position(s).unwrap();
                let mut vec = Vec::new();

                let mut sub_s = &s[1..end];
                while !sub_s.is_empty() {
                    let (elem, new_s) = Self::get_element(sub_s);
                    vec.push(elem);
                    sub_s = new_s;
                }

                let new_s = if end + 2 < s.len() {
                    &s[end + 2..]
                } else {
                    &s[end - 1..end - 1]
                };
                return (Element::List(vec), new_s)
            }
        }
        panic!("get_element({})", std::str::from_utf8(s).unwrap())
    }

    fn compare(a: &Element, b: &Element) -> Ordering {
        match (a, b) {
            (Element::Int(a), Element::Int(b)) => {
                a.cmp(b)
            }
            (Element::List(a), Element::List(b)) => {
                let n = a.len().min(b.len());
                for i in 0..n {
                    let ordering = Self::compare(&a[i], &b[i]);
                    if ordering != Ordering::Equal { return ordering }
                }
                if n < a.len() { Ordering::Greater }
                else if n < b.len() { Ordering::Less }
                else { Ordering::Equal }
            }
            (Element::List(_), Element::Int(value)) => {
                Self::compare(a, &Element::List(vec![Element::Int(*value)]))
            }
            (Element::Int(value), Element::List(_)) => {
                Self::compare(&Element::List(vec![Element::Int(*value)]), b)
            }
        }
    }

    pub fn part_1(s: &str) -> usize {
        let mut lines = Vec::new();
        for line in s.lines().map(|e| e.as_bytes()) {
            if !line.is_empty() {
                lines.push(Self::get_element(line).0);
            }
        }

        let mut r = 0;
        for i in 0..lines.len() / 2 {
            let ii = i * 2;
            if Self::compare(&lines[ii], &lines[ii + 1]) != Ordering::Greater {
                r += i + 1;
            }
        }
        r
    }

    pub fn part_2(s: &str) -> usize {
        let mut lines = Vec::new();
        for line in s.lines().map(|e| e.as_bytes()) {
            if !line.is_empty() {
                lines.push(Self::get_element(line).0);
            }
        }

        let pack_2 = Element::List(vec![Element::List(vec![Element::Int(2)])]);
        let pack_6 = Element::List(vec![Element::List(vec![Element::Int(6)])]);
        lines.push(pack_2.clone());
        lines.push(pack_6.clone());
        lines.sort_unstable_by(|a, b| Self::compare(a, b));

        let mut r = 1;
        for (i, elem) in lines.iter().enumerate() {
            if Self::compare(elem, &pack_2) == Ordering::Equal ||
                Self::compare(elem, &pack_6) == Ordering::Equal {
                r *= i + 1;
            }
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
        test(&Day13::part_1, "input/day_13_0.txt", 13);
    }

    #[test]
    fn part_1_1() {
        test(&Day13::part_1, "input/day_13_1.txt", 5555);
    }

    #[test]
    fn part_2_0() {
        test(&Day13::part_2, "input/day_13_0.txt", 140);
    }

    #[test]
    fn part_2_1() {
        test(&Day13::part_2, "input/day_13_1.txt", 22852);
    }

    fn test(f: &dyn Fn(&str) -> usize, path: &str, r: usize) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}
