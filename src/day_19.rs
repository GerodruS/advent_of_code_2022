use std::collections::HashMap;

struct Day19;

#[derive(Debug)]
struct Prices {
    robots: [Resources;4],
    limits: [u16;4],
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Resources {
    values: [u16;4],
}

impl Resources {
    fn new() -> Resources {
        Self::with(0,0,0,0)
    }

    fn with(ore: u16, clay: u16, obsidian: u16, geode: u16) -> Resources {
        Resources { values: [ore, clay, obsidian, geode] }
    }

    fn gather(&mut self, robots_count: &Resources, duration: u16) {
        for i in 0..4 {
            self.values[i] += robots_count.values[i] * duration;
        }
    }

    fn minus(&mut self, resources: &Resources) {
        for i in 0..4 {
            self.values[i] -= resources.values[i];
        }
    }

    fn get_duration(&self, resources: &Resources, robots_count: &Resources, default: u16) -> u16 {
        let mut max = 0;

        for i in 0..4 {
            if resources.values[i] < self.values[i] {
                if robots_count.values[i] < 1 { return default }
                max = max.max(((self.values[i] - resources.values[i]) as f32 / robots_count.values[i] as f32).ceil() as u16);
            }
        }

        max + 1
    }

    fn less_or_eq(&self, resources: &Resources) -> bool {
        self.values.iter().zip(resources.values.iter()).all(
            |(&a, &b)| a <= b
        )
    }
}

impl Day19 {
    pub fn solve(s: &str, minutes: u16, is_first: bool) -> i32 {
        fn get_max(prices: &Prices, resources: &Resources, robots_count: &Resources, minutes: u16, history: &mut HashMap<(Resources, Resources, u16), u16>) -> u16 {
            let key = (resources.clone(), robots_count.clone(), minutes);
            if let Some(&value) = history.get(&key) {
                return value
            }

            if prices.robots[3].less_or_eq(&robots_count) {
                let max = resources.values[3] + robots_count.values[3] * minutes + minutes * (minutes - 1) / 2;
                history.insert(key, max);
                return max
            }

            let mut max = resources.values[3] + robots_count.values[3] * minutes;

            for i in (0..4).rev() {
                if prices.limits[i] <= robots_count.values[i] { continue }

                let duration = prices.robots[i].get_duration(resources, robots_count, minutes);
                if minutes <= duration { continue }

                let mut new_res = resources.clone();
                new_res.gather(robots_count, duration);
                new_res.minus(&prices.robots[i]);
                let mut new_robots = robots_count.clone();
                new_robots.values[i] += 1;
                let r = get_max(prices, &new_res, &new_robots, minutes - duration, history);
                max = max.max(r);
            }

            history.insert(key, max);
            max
        }

        let prices: Vec<Prices> = s.lines().map(|line| {
            let mut split = line.split(" ");
            let ore = split.nth(6).unwrap().parse().unwrap();
            let clay = split.nth(5).unwrap().parse().unwrap();
            let obsidian = (split.nth(5).unwrap().parse().unwrap(), split.nth(2).unwrap().parse().unwrap());
            let geode = (split.nth(5).unwrap().parse().unwrap(), split.nth(2).unwrap().parse().unwrap());
            Prices {
                robots: [
                    Resources::with(ore, 0, 0, 0),
                    Resources::with(clay, 0, 0, 0),
                    Resources::with(obsidian.0, obsidian.1, 0, 0),
                    Resources::with(geode.0, 0, geode.1, 0),
                ],
                limits: [
                    ore.max(clay).max(obsidian.0).max(geode.0),
                    obsidian.1,
                    geode.1,
                    u16::MAX,
                ],
            }
        }).collect();

        let mut r = 0;

        if is_first {
            let rr: Vec<u16> = prices.iter().enumerate().map(
                |(i, prices)| {
                    let minutes = minutes;
                    let resources = Resources::new();
                    let robots_count = Resources::with(1, 0, 0, 0);
                    let mut history = HashMap::new();
                    let max = get_max(prices, &resources, &robots_count, minutes, &mut history);
                    max * (i as u16 + 1)
                }).collect();
            for max in rr {
                r += max;
            }
        } else {
            let n = 3.min(prices.len());
            r = 1;
            let rr: Vec<u16> = prices[..n].iter().map(
                |prices| {
                    let minutes = minutes;
                    let resources = Resources::new();
                    let robots_count = Resources::with(1, 0, 0, 0);
                    let mut history = HashMap::new();
                    let max = get_max(prices, &resources, &robots_count, minutes, &mut history);
                    max
                }).collect();
            for max in rr {
                r *= max;
            }
        }

        r as i32
    }

    pub fn part_1(s: &str) -> i32 {
        Self::solve(s, 24, true)
    }

    pub fn part_2(s: &str) -> i32 {
        Self::solve(s, 32, false)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day19::part_1, "input/day_19_0.txt", 33);
    }

    #[test]
    fn part_1_1() {
        test(&Day19::part_1, "input/day_19_1.txt", 1834);
    }

    #[test]
    fn part_2_0() {
        test(&Day19::part_2, "input/day_19_0.txt", 56 * 62);
    }

    #[test]
    fn part_2_1() {
        test(&Day19::part_2, "input/day_19_1.txt", 2240);
    }

    fn test(f: &dyn Fn(&str) -> i32, path: &str, r: i32) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}
