use std::collections::VecDeque;

struct Day12;

impl Day12 {
    pub fn part_1(s: &str) -> i32 {
        let mut heights: Vec<Vec<u8>> = s.lines().map(
            |line| line.bytes().collect()
        ).collect();

        let height = heights.len();
        let width = heights[0].len();

        let mut start = (0,0);
        let mut end = (0,0);
        for (y, line) in heights.iter_mut().enumerate() {
            if let Some(x) = line.iter().position(|&x| x == b'S') {
                start = (x,y);
                line[x] = b'a';
            }
            if let Some(x) = line.iter().position(|&x| x == b'E') {
                end = (x,y);
                line[x] = b'z';
            }
        }

        let mut distance = vec![vec![u16::MAX; width]; height];
        distance[start.1][start.0] = 0;

        let mut queue = VecDeque::new();
        queue.push_back(start);

        while !queue.is_empty() {
            let p = queue.pop_front().unwrap();
            let d = distance[p.1][p.0];
            let h = heights[p.1][p.0];

            for delta in [(-1,0), (1,0), (0,-1), (0,1)] {
                let n = ((p.0 as i32 + delta.0) as usize, (p.1 as i32 + delta.1) as usize);
                if n.0 < width &&
                   n.1 < height &&
                   heights[n.1][n.0] <= h + 1 &&
                   d + 1 < distance[n.1][n.0] {
                    distance[n.1][n.0] = d + 1;
                    queue.push_back(n);
                }
            }
        }

        distance[end.1][end.0] as i32
    }

    pub fn part_2(s: &str) -> i32 {
        let mut heights: Vec<Vec<u8>> = s.lines().map(
            |line| line.bytes().collect()
        ).collect();

        let height = heights.len();
        let width = heights[0].len();

        let mut end = (0,0);
        for (y, line) in heights.iter_mut().enumerate() {
            if let Some(x) = line.iter().position(|&x| x == b'S') {
                line[x] = b'a';
            }
            if let Some(x) = line.iter().position(|&x| x == b'E') {
                end = (x,y);
                line[x] = b'z';
            }
        }

        let mut distance = vec![vec![u16::MAX; width]; height];
        let mut queue = VecDeque::new();
        for (y, line) in heights.iter().enumerate() {
            for (x, &h) in line.iter().enumerate() {
                if h == b'a' {
                    distance[y][x] = 0;
                    queue.push_back((x,y));
                }
            }
        }

        while !queue.is_empty() {
            let p = queue.pop_front().unwrap();
            let d = distance[p.1][p.0];
            let h = heights[p.1][p.0];

            for delta in [(-1,0), (1,0), (0,-1), (0,1)] {
                let n = ((p.0 as i32 + delta.0) as usize, (p.1 as i32 + delta.1) as usize);
                if n.0 < width &&
                   n.1 < height &&
                   heights[n.1][n.0] <= h + 1 &&
                   d + 1 < distance[n.1][n.0] {
                    distance[n.1][n.0] = d + 1;
                    queue.push_back(n);
                }
            }
        }

        distance[end.1][end.0] as i32
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day12::part_1, "input/day_12_0.txt", 31);
    }

    #[test]
    fn part_1_1() {
        test(&Day12::part_1, "input/day_12_1.txt", 391);
    }

    #[test]
    fn part_2_0() {
        test(&Day12::part_2, "input/day_12_0.txt", 29);
    }

    #[test]
    fn part_2_1() {
        test(&Day12::part_2, "input/day_12_1.txt", 386);
    }

    fn test(f: &dyn Fn(&str) -> i32, path: &str, r: i32) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}
