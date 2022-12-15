use std::collections::HashSet;

struct Day15;

impl Day15 {
    fn get_sensors(s: &str) -> Vec<(i32, i32, i32, i32)> {
        fn parse_line(line: &str) -> (i32, i32, i32, i32) {
            let mut elems = line
                .trim_start_matches("Sensor at x=")
                .split(": closest beacon is at x=")
                .flat_map(|part| part.split(", y="))
                .map(|elem| elem.parse::<i32>().unwrap());
            (elems.next().unwrap(),
             elems.next().unwrap(),
             elems.next().unwrap(),
             elems.next().unwrap())
        }

        s.lines().map(parse_line).collect()
    }

    pub fn part_1(y: i32, s: &str) -> u64 {
        let sensors = Self::get_sensors(s);
        let beacons_count = sensors.iter()
            .map(|&(_, _, x, y)| (x, y))
            .filter(|&(_, b_y)| b_y == y)
            .collect::<HashSet<_>>().len() as i32;

        let mut ff = Vec::new();
        for &(s_x, s_y, b_x, b_y) in &sensors {
            let d = (b_x - s_x).abs() + (b_y - s_y).abs();
            if s_y - d <= y && y <= s_y + d {
                let a = (y - s_y).abs();
                ff.push((s_x - (d - a), false));
                ff.push((s_x + (d - a), true));
            }
        }

        ff.sort();

        let mut r = 0;
        let mut n = 1;
        let mut prev_x = ff[0].0;
        for &(x, is_end) in ff.iter().skip(1) {
            if is_end {
                n -= 1;
                if n == 0 { r += x - prev_x + 1; }
            } else {
                n += 1;
                if n == 1 { prev_x = x; }
            }
        }

        (r - beacons_count) as u64
    }

    pub fn part_2(max_d: i32, s: &str) -> u64 {
        let sensors: Vec<(i32, i32, i32)> = Self::get_sensors(s).into_iter()
            .map(|b|
                (b.0, b.1, (b.2 - b.0).abs() + (b.3 - b.1).abs())
            ).collect();

        let mut ff = Vec::new();
        for y in 0..=max_d {
            for (b_x, b_y, d) in &sensors {
                if b_y - d <= y && y <= b_y + d {
                    let a = (y - b_y).abs();
                    ff.push((b_x - (d - a), false));
                    ff.push((b_x + (d - a), true));
                }
            }

            ff.sort();

            let mut n = 0;
            for i in 1..ff.len() {
                let (a_x, a_end) = ff[i - 1];
                if a_end {
                    n -= 1;

                    let (b_x, b_end) = ff[i];
                    if n == 0 && !b_end && a_x + 1 < b_x {
                        let x = a_x + 1;
                        return x as u64 * 4000000 + y as u64
                    }
                } else {
                    n += 1;
                }
            }
            ff.clear();
        }

        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day15::part_1, "input/day_15_0.txt", 10, 26);
    }

    #[test]
    fn part_1_1() {
        test(&Day15::part_1, "input/day_15_1.txt", 2000000, 4582667);
    }

    #[test]
    fn part_2_0() {
        test(&Day15::part_2, "input/day_15_0.txt", 20, 56000011);
    }

    #[test]
    fn part_2_1() {
        test(&Day15::part_2, "input/day_15_1.txt", 4000000, 10961118625406);
    }

    fn test(f: &dyn Fn(i32, &str) -> u64, path: &str, y: i32, r: u64) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(y, &input), r);
    }
}
