struct Day11;

impl Day11 {
    pub fn part_1(s: &str) -> usize {
        let mut monkeys = Vec::new();

        {
            let mut lines = s.lines();
            while let Some(_) = lines.next() {
                let line = lines.next().unwrap();
                let items: Vec<usize> = line[18..].split(", ").map(|i| i.parse::<usize>().unwrap()).collect();
                monkeys.push(items);

                lines.next();
                lines.next();
                lines.next();
                lines.next();
                lines.next();
            }
        }

        let mut monkeys_inspect = vec![0; monkeys.len()];

        for _ in 0..20 {
            let mut monkey_i = 0;
            let mut lines = s.lines();
            while let Some(_) = lines.next() {
                lines.next();

                let operation = {
                    let line = lines.next().unwrap();
                    let mut parts = line[23..].split(" ");
                    let operand = parts.next().unwrap();
                    let value = parts.next().unwrap().parse::<usize>();
                    (operand, value)
                };

                let divisible = lines.next().unwrap()[21..].parse::<usize>().unwrap();

                let true_to = lines.next().unwrap()[29..].parse::<usize>().unwrap();
                let false_to = lines.next().unwrap()[30..].parse::<usize>().unwrap();

                let n = monkeys[monkey_i].len();
                monkeys_inspect[monkey_i] += n;

                for i in 0..n {
                    let &item = &monkeys[monkey_i][i];
                    let value = {
                        if let Ok(value) = operation.1 {
                            value
                        } else {
                            item
                        }
                    };

                    let new_value = match operation.0 {
                        "+" => { item + value },
                        "*" => { item * value },
                        _ => { panic!("operation.0={}", operation.0) }
                    };

                    let new_value = new_value / 3;
                    let next_monkey = if new_value % divisible == 0 {
                        true_to
                    } else {
                        false_to
                    };
                    monkeys[next_monkey].push(new_value);
                }
                for _ in 0..n {
                    monkeys[monkey_i].remove(0);
                }

                lines.next();
                monkey_i += 1;
            }
        }

        monkeys_inspect.sort_unstable();
        let a = monkeys_inspect.pop().unwrap();
        let b = monkeys_inspect.pop().unwrap();
        a * b
    }

    pub fn part_2(s: &str) -> usize {
        let mut monkeys = Vec::new();

        let mut m = 1;
        {
            let mut lines = s.lines();
            while let Some(_) = lines.next() {
                let line = lines.next().unwrap();
                let items: Vec<usize> = line[18..].split(", ").map(|i| i.parse::<usize>().unwrap()).collect();
                monkeys.push(items);

                lines.next();
                m *= lines.next().unwrap()[21..].parse::<usize>().unwrap();
                lines.next();
                lines.next();
                lines.next();
            }
        }

        let mut monkeys_inspect = vec![0; monkeys.len()];

        for _ in 0..10_000 {
            let mut monkey_i = 0;
            let mut lines = s.lines();
            while let Some(_) = lines.next() {
                lines.next();

                let operation = {
                    let line = lines.next().unwrap();
                    let mut parts = line[23..].split(" ");
                    let operand = parts.next().unwrap();
                    let value = parts.next().unwrap().parse::<usize>();
                    (operand, value)
                };

                let divisible = lines.next().unwrap()[21..].parse::<usize>().unwrap();

                let true_to = lines.next().unwrap()[29..].parse::<usize>().unwrap();
                let false_to = lines.next().unwrap()[30..].parse::<usize>().unwrap();

                let n = monkeys[monkey_i].len();
                monkeys_inspect[monkey_i] += n;

                for i in 0..n {
                    let &item = &monkeys[monkey_i][i];
                    let value = {
                        if let Ok(value) = operation.1 {
                            value
                        } else {
                            item
                        }
                    };

                    let new_value = match operation.0 {
                        "+" => { item + value },
                        "*" => { item * value },
                        _ => { panic!("operation.0={}", operation.0) }
                    };

                    let new_value = new_value % m;
                    let next_monkey = if new_value % divisible == 0 {
                        true_to
                    } else {
                        false_to
                    };
                    monkeys[next_monkey].push(new_value);
                }
                for _ in 0..n {
                    monkeys[monkey_i].remove(0);
                }

                lines.next();
                monkey_i += 1;
            }
        }

        monkeys_inspect.sort_unstable();
        let a = monkeys_inspect.pop().unwrap();
        let b = monkeys_inspect.pop().unwrap();
        a * b
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day11::part_1, "input/day_11_0.txt", 10605);
    }

    #[test]
    fn part_1_1() {
        test(&Day11::part_1, "input/day_11_1.txt", 58322);
    }

    #[test]
    fn part_2_0() {
        test(&Day11::part_2, "input/day_11_0.txt", 2713310158);
    }

    #[test]
    fn part_2_1() {
        test(&Day11::part_2, "input/day_11_1.txt", 13937702909);
    }

    fn test(f: &dyn Fn(&str) -> usize, path: &str, r: usize) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}
