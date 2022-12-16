use std::collections::HashMap;

struct Day16;

impl Day16 {
    fn parse_data(s: &str) -> (Vec<(i32, Vec<usize>)>, usize) {
        fn parse_line(line: &str) -> (&str, i32, Vec<&str>) {
            let mut split = {
                if line.contains("to valves ") {
                    line.trim_start_matches("Valve ").split("; tunnels lead to valves ")
                } else {
                    line.trim_start_matches("Valve ").split("; tunnel leads to valve ")
                }
            };
            let (valve, rate) = {
                let mut split = split.next().unwrap().split(" has flow rate=");
                let valve = split.next().unwrap();
                let rate = split.next().unwrap().parse::<i32>().unwrap();
                (valve, rate)
            };
            let next_valves: Vec<&str> = split.next().unwrap().split(", ").collect();
            (valve, rate, next_valves)
        }

        let valves: Vec<(&str, i32, Vec<&str>)> = s.lines().map(parse_line).collect();
        let (valves, start_i) = {
            let keys: Vec<&str> = valves.iter().map(|v| v.0).collect();
            let map: Vec<(i32, Vec<usize>)> = valves.iter().map(|(_, rate, next_valves)| {
                let next_valves: Vec<usize> = next_valves.iter().map(|a| keys.iter().position(|v| v == a).unwrap()).collect();
                (*rate, next_valves)
            }).collect();
            (map, keys.iter().position(|&v| v == "AA").unwrap())
        };
        (valves, start_i)
    }

    pub fn part_1(s: &str) -> i32 {
        fn get_max(name: usize, valves: &Vec<(i32, Vec<usize>)>, minutes: i32, opened: u64, history: &mut HashMap<(usize, i32, u64), i32>) -> i32 {
            if let Some(&max) = history.get(&(name, minutes, opened)) {
                return max;
            }

            let (rate, next_valves) = &valves[name];

            let mut max_summ = 0;
            if 0 < *rate && (opened & 1 << name) == 0 {
                // opening
                let opened = opened | 1 << name;
                let minutes = minutes - 1;
                if 3 < minutes {
                    for &v in next_valves {
                        max_summ = max_summ.max(get_max(v, valves, minutes - 1, opened, history));
                    }
                }
                max_summ += *rate * minutes;
            }

            if 3 < minutes {
                // skipping opening
                for &v in next_valves {
                    max_summ = max_summ.max(get_max(v, valves, minutes - 1, opened, history));
                }
            }

            history.insert((name, minutes, opened), max_summ);
            max_summ
        }

        let (valves, start_i) = Self::parse_data(s);
        let max = get_max(start_i, &valves, 30, 0, &mut HashMap::new());
        max
    }

    pub fn part_2(s: &str) -> i32 {
        fn get_max(name_a: usize, name_b: usize, valves: &Vec<(i32, Vec<usize>)>, minutes: i32, opened: u64, history: &mut HashMap<(usize, usize, i32, u64), i32>, non_zero: u32) -> i32 {
            let (name_a, name_b) = (name_a.min(name_b), name_a.max(name_b));
            if let Some(&max) = history.get(&(name_a, name_b, minutes, opened)) {
                return max;
            }

            let (rate_a, next_valves_a) = &valves[name_a];
            let (rate_b, next_valves_b) = &valves[name_b];

            let mut max_summ = 0;
            if name_a != name_b &&
                0 < *rate_a && (opened & 1 << name_a) == 0 &&
                0 < *rate_b && (opened & 1 << name_b) == 0 {
                let opened = opened | 1 << name_a;
                let opened = opened | 1 << name_b;
                let minutes = minutes - 1;
                let mut max = 0;
                if 3 < minutes && opened.count_ones() < non_zero {
                    max = max.max(get_max(name_a, name_b, valves, minutes, opened, history, non_zero));
                }
                max_summ = max_summ.max(max + (*rate_a + *rate_b) * minutes);
            }

            if 0 < *rate_a && (opened & 1 << name_a) == 0 {
                let opened = opened | 1 << name_a;
                let minutes = minutes - 1;
                let mut max = 0;
                if 3 < minutes && opened.count_ones() < non_zero {
                    for &v in next_valves_b {
                        max = max.max(get_max(name_a, v, valves, minutes, opened, history, non_zero));
                    }
                }
                max_summ = max_summ.max(max + *rate_a * minutes);
            }

            if 0 < *rate_b && (opened & 1 << name_b) == 0 {
                let opened = opened | 1 << name_b;
                let minutes = minutes - 1;
                let mut max = 0;
                if 3 < minutes && opened.count_ones() < non_zero {
                    for &v in next_valves_a {
                        max = max.max(get_max(v, name_b, valves, minutes, opened, history, non_zero));
                    }
                }
                max_summ = max_summ.max(max + *rate_b * minutes);
            }

            if 3 < minutes && opened.count_ones() < non_zero {
                for &a in next_valves_a {
                    for &b in next_valves_b {
                        max_summ = max_summ.max(get_max(a, b, valves, minutes - 1, opened, history, non_zero));
                    }
                }
            }

            history.insert((name_a, name_b, minutes, opened), max_summ);
            max_summ
        }

        let (valves, start_i) = Self::parse_data(s);
        let non_zero = valves.iter().filter(|(rate, _)| 0 < *rate).count();
        let max = get_max(start_i, start_i, &valves, 26, 0, &mut HashMap::new(), non_zero as u32);
        max
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day16::part_1, "input/day_16_0.txt", 1651);
    }

    #[test]
    fn part_1_1() {
        test(&Day16::part_1, "input/day_16_1.txt", 2087);
    }

    #[test]
    fn part_2_0() {
        test(&Day16::part_2, "input/day_16_0.txt", 1707);
    }

    #[test]
    fn part_2_1() {
        test(&Day16::part_2, "input/day_16_1.txt", 2591);
    }

    fn test(f: &dyn Fn(&str) -> i32, path: &str, r: i32) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}
