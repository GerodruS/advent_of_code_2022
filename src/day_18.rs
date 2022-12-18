use std::collections::{HashMap, HashSet};

struct Day18;

impl Day18 {
    const D: [(i8, i8, i8); 6] = [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];

    fn parse_input(s: &str) -> HashSet<(i8, i8, i8)> {
        s.lines().map(|line| {
            let mut split = line.split(',').map(|n| n.parse().unwrap());
            let x = split.next().unwrap();
            let y = split.next().unwrap();
            let z = split.next().unwrap();
            (x, y, z)
        }).collect()
    }

    pub fn part_1(s: &str) -> i32 {
        let cubes = Self::parse_input(s);
        cubes.iter().fold(0, |r, &(x, y, z)| {
            Self::D.iter().fold(6, |n, &(dx, dy, dz)| {
                if cubes.contains(&(x - dx, y - dy, z - dz)) {
                    n - 1
                }
                else {
                    n
                }
            }) + r
        })
    }

    pub fn part_2(s: &str) -> i32 {
        fn test_cube((x, y, z): (i8, i8, i8), cubes: &HashSet<(i8, i8, i8)>, bounds: &HashMap<(Option<i8>, Option<i8>, Option<i8>), (i8, i8)>, inside: &mut HashMap<(i8, i8, i8), bool>, visiting: &mut HashSet<(i8, i8, i8)>) -> bool {
            fn test_cube_internal((x, y, z): (i8, i8, i8), cubes: &HashSet<(i8, i8, i8)>, bounds: &HashMap<(Option<i8>, Option<i8>, Option<i8>), (i8, i8)>, inside: &mut HashMap<(i8, i8, i8), bool>, visiting: &mut HashSet<(i8, i8, i8)>) -> bool {
                if let Some(&(min, max)) = bounds.get(&(Some(x), Some(y), None)) {
                    if z <= min || max <= z { return false }
                }
                if let Some(&(min, max)) = bounds.get(&(Some(x), None, Some(z))) {
                    if y <= min || max <= y { return false }
                }
                if let Some(&(min, max)) = bounds.get(&(None, Some(y), Some(z))) {
                    if x <= min || max <= x { return false }
                }

                for &(dx, dy, dz) in &Day18::D {
                    if !test_cube((x + dx, y + dy, z + dz), cubes, bounds, inside, visiting) { return false }
                }
                true
            }

            if visiting.contains(&(x, y, z)) { return true }
            visiting.insert((x, y, z));

            let r = {
                if cubes.contains(&(x, y, z)) { true } else if let Some(&inside) = inside.get(&(x, y, z)) { inside } else {
                    let r = test_cube_internal((x, y, z), cubes, bounds, inside, visiting);
                    inside.insert((x, y, z), r);
                    r
                }
            };

            visiting.remove(&(x, y, z));
            r
        }

        let cubes = Self::parse_input(s);
        let bounds = {
            let mut bounds: HashMap<(Option<i8>, Option<i8>, Option<i8>), (i8, i8)> = HashMap::new();
            for &(x, y, z) in &cubes {
                if let Some(bound) = bounds.get_mut(&(Some(x), Some(y), None)) {
                    bound.0 = bound.0.min(z);
                    bound.1 = bound.1.max(z);
                } else {
                    bounds.insert((Some(x), Some(y), None), (z, z));
                }
                if let Some(bound) = bounds.get_mut(&(Some(x), None, Some(z))) {
                    bound.0 = bound.0.min(y);
                    bound.1 = bound.1.max(y);
                } else {
                    bounds.insert((Some(x), None, Some(z)), (y, y));
                }
                if let Some(bound) = bounds.get_mut(&(None, Some(y), Some(z))) {
                    bound.0 = bound.0.min(x);
                    bound.1 = bound.1.max(x);
                } else {
                    bounds.insert((None, Some(y), Some(z)), (x, x));
                }
            }
            bounds
        };

        let mut inside = HashMap::new();
        let mut visiting = HashSet::new();
        cubes.iter().fold(0, |r, &(x, y, z)| {
            Self::D.iter().fold(6, |n, &(dx, dy, dz)| {
                if test_cube((x - dx, y - dy, z - dz), &cubes, &bounds, &mut inside, &mut visiting) {
                    n - 1
                } else {
                    n
                }
            }) + r
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day18::part_1, "input/day_18_0.txt", 64);
    }

    #[test]
    fn part_1_1() {
        test(&Day18::part_1, "input/day_18_1.txt", 3498);
    }

    #[test]
    fn part_2_0() {
        test(&Day18::part_2, "input/day_18_0.txt", 58);
    }

    #[test]
    fn part_2_1() {
        test(&Day18::part_2, "input/day_18_1.txt", 2008);
    }

    fn test(f: &dyn Fn(&str) -> i32, path: &str, r: i32) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}
