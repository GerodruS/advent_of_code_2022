use std::collections::{HashMap, HashSet};

struct Day23;

impl Day23 {
    const DIRECTIONS: [[(i16, i16); 3]; 4] = [
        [(0, -1), (-1, -1), (1, -1)],
        [(0, 1), (-1, 1), (1, 1)],
        [(-1, 0), (-1, 1), (-1, -1)],
        [(1, 0), (1, 1), (1, -1)],
    ];

    fn parse_positions(s: &str) -> HashSet<(i16, i16)> {
        let mut positions = HashSet::new();
        for (y, line) in s.lines().enumerate().map(|(i, l)| (i, l.as_bytes())) {
            for (x, &c) in line.iter().enumerate() {
                if c == b'#' {
                    positions.insert((x as i16, y as i16));
                }
            }
        }
        positions
    }

    fn do_round(round: usize, positions: &mut HashSet<(i16, i16)>, new_position: &mut HashMap<(i16, i16), Vec<(i16, i16)>>) -> usize {
        for &p in positions.iter() {
            let mut can_stay = true;
            let mut dir = None;
            for i in 0..Self::DIRECTIONS.len() {
                let mut free = true;
                let direction = &Self::DIRECTIONS[(round + i) % Self::DIRECTIONS.len()];
                for &d in direction {
                    let new_p = (p.0 + d.0, p.1 + d.1);
                    free = free && !positions.contains(&new_p);
                }
                can_stay = can_stay && free;
                if free && dir == None { dir = Some(direction[0]) }
            }

            if can_stay { continue }

            if let Some(d) = dir {
                let new_p = (p.0 + d.0, p.1 + d.1);
                if let Some(from) = new_position.get_mut(&new_p) {
                    from.push(p);
                } else {
                    new_position.insert(new_p, vec![p]);
                }
            }
        }

        for (&p, from) in new_position.iter() {
            if from.len() == 1 {
                positions.remove(&from[0]);
                positions.insert(p);
            }
        }

        let n = new_position.len();
        new_position.clear();
        n
    }

    pub fn part_1(s: &str) -> i64 {
        let mut positions = Self::parse_positions(s);
        let mut new_position = HashMap::new();
        for round in 0..10 {
            Self::do_round(round, &mut positions, &mut new_position);
        }

        let mut min = (i16::MAX, i16::MAX);
        let mut max = (i16::MIN, i16::MIN);
        for &p in &positions {
            min = (min.0.min(p.0), min.1.min(p.1));
            max = (max.0.max(p.0), max.1.max(p.1));
        }
        let size = (max.0 - min.0 + 1) as i64 * (max.1 - min.1 + 1) as i64;
        size - positions.len() as i64
    }

    pub fn part_2(s: &str) -> i64 {
        let mut positions = Self::parse_positions(s);
        let mut new_position = HashMap::new();
        for round in 0.. {
            let n = Self::do_round(round, &mut positions, &mut new_position);
            if n == 0 {
                return (round + 1) as i64;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day23::part_1, "input/day_23_0.txt", 110);
    }

    #[test]
    fn part_1_1() {
        test(&Day23::part_1, "input/day_23_1.txt", 4302);
    }

    #[test]
    fn part_2_0() {
        test(&Day23::part_2, "input/day_23_0.txt", 20);
    }

    #[test]
    fn part_2_1() {
        test(&Day23::part_2, "input/day_23_1.txt", 1025);
    }

    fn test(f: &dyn Fn(&str) -> i64, path: &str, r: i64) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}