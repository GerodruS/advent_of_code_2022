use std::collections::{HashSet, VecDeque};

struct Day24;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
enum Direction {
    Up = 0b0001,
    Down = 0b0010,
    Left = 0b0100,
    Right = 0b1000,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Cell {
    None,
    Wall,
    Wind(u8),
}

impl Day24 {
    /*
    fn print_field(field: &Vec<Vec<Cell>>) {
        for line in field {
            for &c in line {
                match c {
                    Cell::None => print!("."),
                    Cell::Wall => print!("#"),
                    Cell::Wind(w) => {
                        let n = w.count_ones();
                        if 1 < n { print!("{n}"); }
                        else if w == Direction::Up as u8 { print!("^") }
                        else if w == Direction::Down as u8 { print!("v") }
                        else if w == Direction::Left as u8 { print!("<") }
                        else if w == Direction::Right as u8 { print!(">") }
                    }
                }
            }
            println!();
        }
        println!();
    }
    */

    fn add_wind((mut x, mut y): (usize, usize), w: Direction, prev: &Vec<Vec<Cell>>, next: &mut Vec<Vec<Cell>>) {
        while {
            (x, y) = match w {
                Direction::Up => if y == 0 {
                    (x, prev.len() - 1)
                } else {
                    (x, y - 1)
                }
                Direction::Down => if y == prev.len() - 1 {
                    (x, 0)
                } else {
                    (x, y + 1)
                }
                Direction::Left => if x == 0 {
                    (prev[y].len() - 1, y)
                } else {
                    (x - 1, y)
                }
                Direction::Right => if x == prev[y].len() - 1 {
                    (0, y)
                } else {
                    (x + 1, y)
                }
            };
            prev[y][x] == Cell::Wall
        } {}

        match next[y][x] {
            Cell::Wind(value) => next[y][x] = Cell::Wind(value | w as u8),
            _ => next[y][x] = Cell::Wind(w as u8),
        }
    }

    fn simulate(prev: &Vec<Vec<Cell>>, next: &mut Vec<Vec<Cell>>) {
        next.iter_mut().for_each(|l| l.fill(Cell::None));
        for (y, line) in prev.iter().enumerate() {
            for (x, &c) in line.iter().enumerate() {
                match c {
                    Cell::None => {}
                    Cell::Wall => next[y][x] = Cell::Wall,
                    Cell::Wind(w) => {
                        if w & Direction::Up as u8 != 0 { Self::add_wind((x, y), Direction::Up, prev, next) }
                        if w & Direction::Down as u8 != 0 { Self::add_wind((x, y), Direction::Down, prev, next) }
                        if w & Direction::Left as u8 != 0 { Self::add_wind((x, y), Direction::Left, prev, next) }
                        if w & Direction::Right as u8 != 0 { Self::add_wind((x, y), Direction::Right, prev, next) }
                    }
                }
            }
        }
    }

    fn can_move((x, y): (usize, usize), field: &Vec<Vec<Cell>>) -> bool {
        field[y][x] == Cell::None
    }

    pub fn solve(s: &str, journey_count: u8) -> i64 {
        let field: Vec<Vec<Cell>> = s.lines().map(
            |l| l.as_bytes().iter().map(|&c| {
                match c {
                    b'.' => Cell::None,
                    b'#' => Cell::Wall,
                    b'^' => Cell::Wind(Direction::Up as u8),
                    b'v' => Cell::Wind(Direction::Down as u8),
                    b'<' => Cell::Wind(Direction::Left as u8),
                    b'>' => Cell::Wind(Direction::Right as u8),
                    _ => panic!("c='{c}'")
                }
            }).collect()
        ).collect();
        let size = (field[0].len(), field.len());
        let mut fields = [field, vec![vec![Cell::None;size.0];size.1]];
        let mut current_i = 0;
        let start_position = (fields[current_i][0].iter().position(|&c| c == Cell::None).unwrap(), 0);
        let mut positions = VecDeque::new();
        let mut next_positions = HashSet::new();
        positions.push_back((start_position, 0));

        for i in 0.. {
            let (a, b) = fields.split_at_mut(1);
            let (prev_f, next_f) = if current_i == 0 { (&mut a[0], &mut b[0]) } else { (&mut b[0], &mut a[0]) };
            current_i = (current_i + 1) % 2;
            Self::simulate(prev_f, next_f);

            let n = positions.len();
            if n == 0 { panic!("round {i}") }

            for _ in 0..n {
                let (p, mut goal) = positions.pop_front().unwrap();
                if p.1 == next_f.len() - 1 {
                    if goal == journey_count {
                        return i
                    } else if goal % 2 == 0 {
                        goal += 1;
                    }
                } else if p.1 == 0 {
                    if goal % 2 == 1 {
                        goal += 1;
                    }
                }

                if Self::can_move(p, next_f) {
                    next_positions.insert((p, goal));
                }
                if p.1 != 0 && Self::can_move((p.0, p.1 - 1), next_f) {
                    next_positions.insert(((p.0, p.1 - 1), goal));
                }
                if p.1 != next_f.len() - 1 && Self::can_move((p.0, p.1 + 1), next_f) {
                    next_positions.insert(((p.0, p.1 + 1), goal));
                }
                if Self::can_move((p.0 - 1, p.1), next_f) {
                    next_positions.insert(((p.0 - 1, p.1), goal));
                }
                if Self::can_move((p.0 + 1, p.1), next_f) {
                    next_positions.insert(((p.0 + 1, p.1), goal));
                }
            }

            for &elem in &next_positions {
                positions.push_back(elem);
            }
            next_positions.clear();
        }

        unreachable!()
    }

    pub fn part_1(s: &str) -> i64 {
        Self::solve(s, 0)
    }

    pub fn part_2(s: &str) -> i64 {
        Self::solve(s, 2)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day24::part_1, "input/day_24_0.txt", 18);
    }

    #[test]
    fn part_1_1() {
        test(&Day24::part_1, "input/day_24_1.txt", 295);
    }

    #[test]
    fn part_2_0() {
        test(&Day24::part_2, "input/day_24_0.txt", 54);
    }

    #[test]
    fn part_2_1() {
        test(&Day24::part_2, "input/day_24_1.txt", 851);
    }

    fn test(f: &dyn Fn(&str) -> i64, path: &str, r: i64) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}