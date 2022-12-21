use std::collections::HashMap;

struct Day21;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Monkey {
    Value(i64),
    Sum(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Div(usize, usize),
}

impl Day21 {
    fn parse_monkeys(s: &str) -> (HashMap<&str, usize>, Vec<Monkey>) {
        let mut id_to_i = HashMap::new();
        let mut expressions = Vec::new();
        for (i, line) in s.lines().enumerate() {
            let mut split = line.split(": ");
            let id = split.next().unwrap();
            let expression = split.next().unwrap();
            id_to_i.insert(id, i);
            expressions.push(expression);
        }

        let mut monkeys = Vec::new();
        for expression in expressions {
            if let Ok(value) = expression.parse() {
                monkeys.push(Monkey::Value(value));
            } else if expression.contains('+') {
                let mut split = expression.split(" + ");
                let a = split.next().unwrap();
                let b = split.next().unwrap();
                let a = id_to_i[a];
                let b = id_to_i[b];
                monkeys.push(Monkey::Sum(a, b));
            } else if expression.contains('-') {
                let mut split = expression.split(" - ");
                let a = split.next().unwrap();
                let b = split.next().unwrap();
                let a = id_to_i[a];
                let b = id_to_i[b];
                monkeys.push(Monkey::Sub(a, b));
            } else if expression.contains('*') {
                let mut split = expression.split(" * ");
                let a = split.next().unwrap();
                let b = split.next().unwrap();
                let a = id_to_i[a];
                let b = id_to_i[b];
                monkeys.push(Monkey::Mul(a, b));
            } else if expression.contains('/') {
                let mut split = expression.split(" / ");
                let a = split.next().unwrap();
                let b = split.next().unwrap();
                let a = id_to_i[a];
                let b = id_to_i[b];
                monkeys.push(Monkey::Div(a, b));
            } else {
                panic!("expression={expression}");
            }
        }

        (id_to_i, monkeys)
    }

    fn solve(i: usize, monkeys: &Vec<Monkey>) -> i64 {
        match &monkeys[i] {
            &Monkey::Value(value) => value,
            &Monkey::Sum(a, b) => Self::solve(a, monkeys) + Self::solve(b, monkeys),
            &Monkey::Sub(a, b) => Self::solve(a, monkeys) - Self::solve(b, monkeys),
            &Monkey::Mul(a, b) => Self::solve(a, monkeys) * Self::solve(b, monkeys),
            &Monkey::Div(a, b) => Self::solve(a, monkeys) / Self::solve(b, monkeys),
        }
    }

    fn solve_backward(monkey: usize, monkeys: &Vec<Monkey>, root: usize) -> i64 {
        let i = monkeys.iter().position(|m|
            match m {
                &Monkey::Sum(a, b) => monkey == a || monkey == b,
                &Monkey::Sub(a, b) => monkey == a || monkey == b,
                &Monkey::Mul(a, b) => monkey == a || monkey == b,
                &Monkey::Div(a, b) => monkey == a || monkey == b,
                _ => false,
            }
        ).unwrap();

        if i == root {
            match &monkeys[i] {
                &Monkey::Sum(a, b) => {
                    if a == monkey {
                        Self::solve(b, monkeys)
                    } else { // b == monkey
                        Self::solve(a, monkeys)
                    }
                },
                &Monkey::Sub(a, b) => {
                    if a == monkey {
                        Self::solve(b, monkeys)
                    } else { // b == monkey
                        Self::solve(a, monkeys)
                    }
                },
                &Monkey::Mul(a, b) => {
                    if a == monkey {
                        Self::solve(b, monkeys)
                    } else { // b == monkey
                        Self::solve(a, monkeys)
                    }
                },
                &Monkey::Div(a, b) => {
                    if a == monkey {
                        Self::solve(b, monkeys)
                    } else { // b == monkey
                        Self::solve(a, monkeys)
                    }
                },
                _ => panic!("{:?}", monkeys[i])
            }
        } else {
            match &monkeys[i] {
                &Monkey::Sum(a, b) => {
                    if a == monkey {
                        Self::solve_backward(i, monkeys, root) - Self::solve(b, monkeys)
                    } else { // b == monkey
                        Self::solve_backward(i, monkeys, root) - Self::solve(a, monkeys)
                    }
                },
                &Monkey::Sub(a, b) => {
                    if a == monkey {
                        Self::solve_backward(i, monkeys, root) + Self::solve(b, monkeys)
                    } else { // b == monkey
                        Self::solve(a, monkeys) - Self::solve_backward(i, monkeys, root)
                    }
                },
                &Monkey::Mul(a, b) => {
                    if a == monkey {
                        Self::solve_backward(i, monkeys, root) / Self::solve(b, monkeys)
                    } else { // b == monkey
                        Self::solve_backward(i, monkeys, root) / Self::solve(a, monkeys)
                    }
                },
                &Monkey::Div(a, b) => {
                    if a == monkey {
                        Self::solve_backward(i, monkeys, root) * Self::solve(b, monkeys)
                    } else { // b == monkey
                        Self::solve(a, monkeys) / Self::solve_backward(i, monkeys, root)
                    }
                },
                _ => panic!("{:?}", monkeys[i])
            }
        }
    }

    pub fn part_1(s: &str) -> i64 {
        let (id_to_i, monkeys) = Self::parse_monkeys(s);
        let root = id_to_i["root"];
        Self::solve(root, &monkeys)
    }

    pub fn part_2(s: &str) -> i64 {
        let (id_to_i, monkeys) = Self::parse_monkeys(s);
        let root = id_to_i["root"];
        let humn = id_to_i["humn"];
        Self::solve_backward(humn, &monkeys, root)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day21::part_1, "input/day_21_0.txt", 152);
    }

    #[test]
    fn part_1_1() {
        test(&Day21::part_1, "input/day_21_1.txt", 158731561459602);
    }

    #[test]
    fn part_2_0() {
        test(&Day21::part_2, "input/day_21_0.txt", 301);
    }

    #[test]
    fn part_2_1() {
        test(&Day21::part_2, "input/day_21_1.txt", 3769668716709);
    }

    fn test(f: &dyn Fn(&str) -> i64, path: &str, r: i64) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}