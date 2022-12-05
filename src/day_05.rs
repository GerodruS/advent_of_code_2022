struct Day05;

impl Day05 {
    pub fn part_1(s: &str) -> String {
        let (stack_h, _) = s.lines().enumerate().find(|(_, str)| str.starts_with(" 1")).unwrap();

        let lines = s.lines();
        let stacks_n = (lines.clone().nth(0).unwrap().len() + 1) / 4;
        let mut stacks = vec![Vec::new(); stacks_n];
        for line_i in (0..stack_h).rev() {
            let line = lines.clone().nth(line_i).unwrap();
            for stack_i in 0..stacks_n {
                let elem = line.bytes().nth(1 + stack_i * 4).unwrap();
                if elem != b' ' {
                    stacks[stack_i].push(elem);
                }
            }
        }

        for line in lines.skip(stack_h + 2) {
            let mut split = line.split(' ');

            split.next();
            let count = split.next().unwrap().parse::<usize>().unwrap();
            split.next();
            let from = split.next().unwrap().parse::<usize>().unwrap() - 1;
            split.next();
            let to = split.next().unwrap().parse::<usize>().unwrap() - 1;

            for _ in 0..count {
                let elem = stacks[from].pop().unwrap();
                stacks[to].push(elem);
            }
        }

        stacks.into_iter().map(|stack| stack.last().unwrap().clone() as char).collect()
    }

    pub fn part_2(s: &str) -> String {
        let (stack_h, _) = s.lines().enumerate().find(|(_, str)| str.starts_with(" 1")).unwrap();

        let lines = s.lines();
        let stacks_n = (lines.clone().nth(0).unwrap().len() + 1) / 4;
        let mut stacks = vec![Vec::new(); stacks_n];
        for line_i in (0..stack_h).rev() {
            let line = lines.clone().nth(line_i).unwrap();
            for stack_i in 0..stacks_n {
                let elem = line.bytes().nth(1 + stack_i * 4).unwrap();
                if elem != b' ' {
                    stacks[stack_i].push(elem);
                }
            }
        }

        for line in lines.skip(stack_h + 2) {
            let mut split = line.split(' ');

            split.next();
            let count = split.next().unwrap().parse::<usize>().unwrap();
            split.next();
            let from = split.next().unwrap().parse::<usize>().unwrap() - 1;
            split.next();
            let to = split.next().unwrap().parse::<usize>().unwrap() - 1;

            let n = stacks[from].len();
            for i in n-count..n {
                let elem = stacks[from][i].clone();
                stacks[to].push(elem);
            }
            stacks[from].resize(n - count, 0);
        }

        stacks.into_iter().map(|stack| stack.last().unwrap().clone() as char).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day05::part_1, "input/day_05_0.txt", "CMZ");
    }

    #[test]
    fn part_1_1() {
        test(&Day05::part_1, "input/day_05_1.txt", "VGBBJCRMN");
    }

    #[test]
    fn part_2_0() {
        test(&Day05::part_2, "input/day_05_0.txt", "MCD");
    }

    #[test]
    fn part_2_1() {
        test(&Day05::part_2, "input/day_05_1.txt", "LBBVJBRMH");
    }

    fn test(f: &dyn Fn(&str) -> String, path: &str, r: &str) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}
