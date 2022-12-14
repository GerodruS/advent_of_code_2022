use std::collections::HashSet;

struct Day14;

impl Day14 {
    const START_P: (usize, usize) = (500, 0);

    fn parse_line(line: &str) -> Vec<(usize, usize)> {
        fn parse_element(elem: &str) -> (usize, usize) {
            let mut split = elem.split(',');
            let x = split.next().unwrap().parse::<usize>().unwrap();
            let y = split.next().unwrap().parse::<usize>().unwrap();
            (x, y)
        }

        line.split(" -> ").map(parse_element).collect()
    }

    fn get_bounds(lines: &Vec<Vec<(usize, usize)>>) -> (usize, usize, usize, usize) {
        let max_x = lines.iter().flatten().max_by(|a, b| a.0.partial_cmp(&b.0).unwrap()).unwrap().0;
        let min_x = lines.iter().flatten().min_by(|a, b| a.0.partial_cmp(&b.0).unwrap()).unwrap().0;
        let max_y = lines.iter().flatten().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().1;
        let min_y = lines.iter().flatten().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().1;

        let max_x = max_x.max(Self::START_P.0);
        let min_x = min_x.min(Self::START_P.0);
        let max_y = max_y.max(Self::START_P.1);
        let min_y = min_y.min(Self::START_P.1);

        (min_x, min_y, max_x, max_y)
    }

    pub fn part_1(s: &str) -> usize {
        let mut lines: Vec<Vec<(usize, usize)>> = s.lines().map(Self::parse_line).collect();
        let (min_x, min_y, max_x, max_y) = Self::get_bounds(&lines);

        lines.iter_mut().for_each(|line| line.iter_mut().for_each(|p| {
            p.0 -= min_x;
            p.1 -= min_y;
        }));
        let max_x = max_x - min_x;
        let max_y = max_y - min_y;
        let size = (max_x + 1, max_y + 1);
        let start_p = (Self::START_P.0 - min_x, Self::START_P.1 - min_y);

        let mut field = vec![vec![false; size.0]; size.1];
        for line in &lines {
            let mut a = line[0];
            for &b in line.iter().skip(1) {
                let d = (b.0 as i32 - a.0 as i32, b.1 as i32 - a.1 as i32);
                let d = (d.0.signum(), d.1.signum());
                while a != b {
                    field[a.1][a.0] = true;
                    a = ((a.0 as i32 + d.0) as usize, (a.1 as i32 + d.1) as usize);
                }
                field[a.1][a.0] = true;
            }
        }

        let mut n = 0;
        let mut next = true;
        while next {
            n += 1;
            let mut unit = start_p;
            let fall = loop {
                if unit.1 + 1 == size.1 {
                    break true
                } else if !field[unit.1 + 1][unit.0] {
                    unit.1 += 1;
                } else if unit.0 == 0 {
                    break true
                } else if !field[unit.1 + 1][unit.0 - 1] {
                    unit.0 -= 1;
                    unit.1 += 1;
                } else if unit.0 + 1 == size.0 {
                    break true
                } else if !field[unit.1 + 1][unit.0 + 1] {
                    unit.0 += 1;
                    unit.1 += 1;
                } else {
                    field[unit.1][unit.0] = true;
                    break false
                }
            };
            next = !fall;

        }
        n - 1
    }

    pub fn part_2(s: &str) -> usize {
        let mut lines: Vec<Vec<(usize, usize)>> = s.lines().map(Self::parse_line).collect();
        let (_, min_y, _, max_y) = Self::get_bounds(&lines);

        let min_x = 0;
        let max_y = max_y + 2;

        lines.iter_mut().for_each(|line| line.iter_mut().for_each(|p| {
            p.0 -= min_x;
            p.1 -= min_y;
        }));
        let max_y = max_y - min_y;
        let start_p = (Self::START_P.0 - min_x, Self::START_P.1 - min_y);

        let mut field = HashSet::new();
        for line in &lines {
            let mut a = line[0];
            for &b in line.iter().skip(1) {
                let d = (b.0 as i32 - a.0 as i32, b.1 as i32 - a.1 as i32);
                let d = (d.0.signum(), d.1.signum());
                while a != b {
                    field.insert(a);
                    a = ((a.0 as i32 + d.0) as usize, (a.1 as i32 + d.1) as usize);
                }
                field.insert(a);
            }
        }

        let mut n = 0;
        while !field.contains(&start_p) {
            n += 1;
            let mut unit = start_p;
            loop {
                if unit.1 + 1 == max_y {
                    field.insert(unit);
                    break
                } else if !field.contains(&(unit.0, unit.1 + 1)) {
                    unit.1 += 1;
                } else if unit.0 == 0 {
                    break
                } else if !field.contains(&(unit.0 - 1, unit.1 + 1)) {
                    unit.0 -= 1;
                    unit.1 += 1;
                } else if !field.contains(&(unit.0 + 1, unit.1 + 1)) {
                    unit.0 += 1;
                    unit.1 += 1;
                } else {
                    field.insert(unit);
                    break
                }
            };
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
        test(&Day14::part_1, "input/day_14_0.txt", 24);
    }

    #[test]
    fn part_1_1() {
        test(&Day14::part_1, "input/day_14_1.txt", 979);
    }

    #[test]
    fn part_2_0() {
        test(&Day14::part_2, "input/day_14_0.txt", 93);
    }

    #[test]
    fn part_2_1() {
        test(&Day14::part_2, "input/day_14_1.txt", 29044);
    }

    fn test(f: &dyn Fn(&str) -> usize, path: &str, r: usize) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}
