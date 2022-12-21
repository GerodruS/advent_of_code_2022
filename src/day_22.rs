use std::collections::HashMap;

struct Day22;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Instruction {
    Move(usize),
    Rotate(bool), // is_right
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl From<u8> for Direction {
    fn from(x: u8) -> Self {
        match x {
            0 => return Direction::Right,
            1 => return Direction::Down,
            2 => return Direction::Left,
            3 => return Direction::Up,
            _ => panic!("x={x}"),
        };
    }
}

impl Day22 {
    pub fn part_1(s: &str) -> i64 {
        let instructions = {
            let instructions_line = s.lines().last().unwrap().as_bytes();
            let mut i = 0;
            let mut instructions = Vec::new();
            while i < instructions_line.len() {
                let next_r = i + instructions_line[i..].iter().position(|&c| c == b'L' || c == b'R').unwrap_or(instructions_line.len() - i);
                let steps = std::str::from_utf8(&instructions_line[i..next_r]).unwrap().parse().unwrap();
                instructions.push(Instruction::Move(steps));
                if next_r < instructions_line.len() {
                    instructions.push(Instruction::Rotate(instructions_line[next_r] == b'R'));
                }
                i = next_r + 1;
            }
            instructions
        };
        let field: Vec<&[u8]> = s.lines().map(|line| line.as_bytes()).collect();
        let field = &field[0..field.len() - 2];
        let height = field.len() as i32;

        let start = (field[0].iter().position(|&c| c == b'.').unwrap() as i32, 0);
        let mut pos = start;
        let mut dir = Direction::Right;
        let mut move_count = 0;

        for instruction in instructions {
            match instruction {
                Instruction::Move(n) => {
                    move_count = n;
                },
                Instruction::Rotate(is_right) => {
                    if is_right {
                        dir = ((dir as u8 + 1) % 4).into();
                    } else {
                        dir = ((dir as u8 + 3) % 4).into();
                    }
                },
            }
            while 0 < move_count {
                move_count -= 1;
                match dir {
                    Direction::Right => {
                        let line = field[pos.1 as usize];
                        {
                            let mut p = (pos.0 + 1, pos.1);
                            if pos.0 + 1 == line.len() as i32 || line[p.0 as usize] == b' ' {
                                p.0 = line.iter().position(|&c| c != b' ').unwrap() as i32;
                            }
                            if line[p.0 as usize] == b'.' {
                                pos = p;
                            } else if line[p.0 as usize] == b'#' {
                                // break
                            } else {
                                panic!("{:?} {}", p, line[p.0 as usize] as char);
                            }
                        }
                    }
                    Direction::Down => {
                        {
                            let mut p = (pos.0, pos.1 + 1);
                            if pos.1 + 1 == field.len() as i32 || field[p.1 as usize].len() as i32 <= p.0 || field[p.1 as usize][p.0 as usize] == b' ' {
                                for y in 0..height as usize {
                                    if pos.0 < field[y].len() as i32 && field[y][pos.0 as usize] != b' ' {
                                        p.1 = y as i32;
                                        break;
                                    }
                                }
                            }
                            if field[p.1 as usize][p.0 as usize] == b'.' {
                                pos = p;
                            } else if field[p.1 as usize][p.0 as usize] == b'#' {
                                // break
                            } else {
                                panic!("{:?} {}", p, field[p.1 as usize][p.0 as usize] as char);
                            }
                        }
                    }
                    Direction::Left => {
                        let line = field[pos.1 as usize];
                        {
                            let mut p = (pos.0 - 1, pos.1);
                            if pos.0 == 0 || line[p.0 as usize] == b' ' {
                                p.0 = line.iter().rposition(|&c| c != b' ').unwrap() as i32;
                            }
                            if line[p.0 as usize] == b'.' {
                                pos = p;
                            } else if line[p.0 as usize] == b'#' {
                                // break
                            } else {
                                panic!("{:?} {}", p, line[p.0 as usize] as char);
                            }
                        }
                    }
                    Direction::Up => {
                        {
                            let mut p = (pos.0, pos.1 - 1);
                            if pos.1 == 0 as i32 || field[p.1 as usize].len() as i32 <= p.0 || field[p.1 as usize][p.0 as usize] == b' ' {
                                for y in (0..height as usize).rev() {
                                    if pos.0 < field[y].len() as i32 && field[y][pos.0 as usize] != b' ' {
                                        p.1 = y as i32;
                                        break;
                                    }
                                }
                            }
                            if field[p.1 as usize][p.0 as usize] == b'.' {
                                pos = p;
                            } else if field[p.1 as usize][p.0 as usize] == b'#' {
                                // break
                            } else {
                                panic!("{:?} {}", p, field[p.1 as usize][p.0 as usize] as char);
                            }
                        }
                    }
                };
            }
        }

        let x = pos.0 as i64 + 1;
        let y = pos.1 as i64 + 1;
        let d = dir as i64;
        // println!("{} {} {}", x, y, d);
        1000 * y + 4 * x + d
    }

    pub fn part_2(s: &str) -> i64 {
        let instructions = {
            let instructions_line = s.lines().last().unwrap().as_bytes();
            let mut i = 0;
            let mut instructions = Vec::new();
            while i < instructions_line.len() {
                let next_r = i + instructions_line[i..].iter().position(|&c| c == b'L' || c == b'R').unwrap_or(instructions_line.len() - i);
                let steps = std::str::from_utf8(&instructions_line[i..next_r]).unwrap().parse().unwrap();
                instructions.push(Instruction::Move(steps));
                if next_r < instructions_line.len() {
                    instructions.push(Instruction::Rotate(instructions_line[next_r] == b'R'));
                }
                i = next_r + 1;
            }
            instructions
        };
        let field: Vec<&[u8]> = s.lines().map(|line| line.as_bytes()).collect();

        let field = &field[0..field.len() - 2];
        let dimension = if 50 < field.len() { 50 } else { 4 };
        let mut map = HashMap::new();
        let field = {
            let mut f = vec![vec![b' '; 4 * dimension as usize]; 3 * dimension as usize];
            if dimension == 4 {
                for y in 0..3 * dimension {
                    for x in 0..3 * dimension {
                        f[y][x] = field[y][x];
                        map.insert((x, y), (x, y));
                    }
                }
                for y in 0..dimension {
                    for x in 0..dimension {
                        f[y + dimension][x + 3 * dimension] = field[x + 2 * dimension][4 * dimension - y - 1];
                        map.insert((4 * dimension - y - 1, x + 2 * dimension), (x + 3 * dimension, y + dimension));
                    }
                }
            } else {
                for y in 0..3 * dimension {
                    for x in 0..dimension {
                        f[y][x + 2 * dimension] = field[y][x + dimension];
                        map.insert((x + 2 * dimension, y), (x + dimension, y));
                    }
                }
                for y in 0..dimension {
                    for x in 0..dimension {
                        f[y + dimension][x + 3 * dimension] = field[dimension - x - 1][y + 2 * dimension];
                        map.insert((y + dimension, y + dimension), (dimension - x - 1, y + 2 * dimension));
                    }
                }
                for y in 0..dimension {
                    for x in 0..2*dimension {
                        f[y + dimension][x] = field[4*dimension - x - 1][y];
                        map.insert((x, y + dimension), (4*dimension - x - 1, y));
                    }
                }
            }
            f
        };
        let dimension = dimension as i32;
        // for l in &field {
        //     println!("{:?}", std::str::from_utf8(&l).unwrap());
        // }
        //
        // let height = field.len() as i32;
        // let width = field.iter().map(|l| l.len()).max().unwrap() as i32;

        let start = (field[0].iter().position(|&c| c == b'.').unwrap() as i32, 0_i32);
        let mut pos = start;
        let mut dir = Direction::Right;
        let mut move_count = 0;

        for instruction in instructions {
            match instruction {
                Instruction::Move(n) => {
                    move_count = n;
                },
                Instruction::Rotate(is_right) => {
                    if is_right {
                        dir = ((dir as u8 + 1) % 4).into();
                    } else {
                        dir = ((dir as u8 + 3) % 4).into();
                    }
                },
            }
            while 0 < move_count {
                move_count -= 1;
                let prev_dir = dir;
                match dir {
                    Direction::Right => {
                        {
                            let mut p = (pos.0 + 1, pos.1);
                            if pos.0 + 1 == field[pos.1 as usize].len() as i32 {
                                p.0 = 0;
                            } else if field[p.1 as usize][p.0 as usize] == b' ' {
                                let (_, y) = pos;
                                p = if pos.1 / dimension == 0 {
                                    dir = Direction::Down;
                                    (dimension - y - 1 + 3 * dimension, dimension)
                                } else {
                                    dir = Direction::Up;
                                    (y + dimension, 2 * dimension - 1)
                                };
                            }
                            if field[p.1 as usize][p.0 as usize] == b'.' {
                                pos = p;
                            } else if field[p.1 as usize][p.0 as usize] == b'#' {
                                move_count = 0;
                                dir = prev_dir;
                            } else {
                                panic!("{:?} {:?} '{}'", pos, p, field[p.1 as usize][p.0 as usize] as char);
                            }
                        }
                    }
                    Direction::Down => {
                        {
                            let mut p = (pos.0, pos.1 + 1);
                            let (x, _) = pos;
                            if pos.1 + 1 == field.len() as i32 {
                                dir = Direction::Up;
                                p = (3 * dimension - x - 1, 2 * dimension - 1);
                            } else if field[p.1 as usize][p.0 as usize] == b' ' {
                                p = if pos.0 / dimension == 0 {
                                    dir = Direction::Up;
                                    (3 * dimension - x - 1, field.len() as i32 - 1)
                                } else if pos.0 / dimension == 1 {
                                    dir = Direction::Right;
                                    (2 * dimension, 2 * dimension + (2 * dimension - x - 1))
                                } else {
                                    dir = Direction::Left;
                                    (3 * dimension - 1, x - dimension)
                                };
                            }
                            if field[p.1 as usize][p.0 as usize] == b'.' {
                                pos = p;
                            } else if field[p.1 as usize][p.0 as usize] == b'#' {
                                move_count = 0;
                                dir = prev_dir;
                            } else {
                                panic!("{:?} {}", p, field[p.1 as usize][p.0 as usize] as char);
                            }
                        }
                    }
                    Direction::Left => {
                        {
                            let mut p = (pos.0 - 1, pos.1);
                            if pos.0 == 0 {
                                p.0 = field[pos.1 as usize].len() as i32 - 1;
                            } else if field[pos.1 as usize][p.0 as usize] == b' ' {
                                let (_, y) = pos;
                                p = if pos.1 / dimension == 0 {
                                    dir = Direction::Down;
                                    (y + dimension, dimension)
                                } else {
                                    dir = Direction::Up;
                                    (2 * dimension - (y - 2 * dimension) - 1, 2 * dimension - 1)
                                };
                            }
                            // println!("{:?} {:?}", pos, p);
                            if field[p.1 as usize][p.0 as usize] == b'.' {
                                pos = p;
                            } else if field[p.1 as usize][p.0 as usize] == b'#' {
                                move_count = 0;
                                dir = prev_dir;
                            } else {
                                panic!("{:?} {}", p, field[p.1 as usize][p.0 as usize] as char);
                            }
                        }
                    }
                    Direction::Up => {
                        {
                            let mut p = (pos.0, pos.1 - 1);
                            let (x, _) = pos;
                            if pos.1 == 0 {
                                dir = Direction::Down;
                                p = (3 * dimension - x - 1, dimension);
                            } else if field[p.1 as usize][p.0 as usize] == b' ' {
                                p = if pos.0 / dimension == 0 {
                                    dir = Direction::Down;
                                    (3 * dimension - x - 1, 0)
                                } else if pos.0 / dimension == 1 {
                                    dir = Direction::Right;
                                    (2 * dimension, x - dimension)
                                } else {
                                    dir = Direction::Left;
                                    (3 * dimension - 1, dimension - (x - 3 * dimension) - 1)
                                };
                            }
                            // println!("{:?} {:?}", pos, p);
                            if field[p.1 as usize][p.0 as usize] == b'.' {
                                pos = p;
                            } else if field[p.1 as usize][p.0 as usize] == b'#' {
                                move_count = 0;
                                dir = prev_dir;
                            } else {
                                panic!("{:?} {}", p, field[p.1 as usize][p.0 as usize] as char);
                            }
                        }
                    }
                };

                // for y in 0..3*dimension {
                //     for x in 0..4*dimension {
                //         if pos == (x, y) {
                //             print!("@");
                //         } else {
                //             print!("{}", field[y as usize][x as usize] as char);
                //         }
                //     }
                //     println!();
                // }
                // println!();
            }
        }

        let x = pos.0 as i64 + 1;
        let y = pos.1 as i64 + 1;
        let d = dir as i64;
        // println!("- {} {} {}", x, y, d);
        let &(x, y) = map.get(&(x as usize, y as usize)).unwrap();
        // println!("+ {} {} {}", x, y, d);
        1000 * y as i64 + 4 * x as i64 + d
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day22::part_1, "input/day_22_0.txt", 6032); // 8 6 0
    }

    #[test]
    fn part_1_1() {
        test(&Day22::part_1, "input/day_22_1.txt", 60362); // 90 60 2
    }

    #[test]
    fn part_2_0() {
        test(&Day22::part_2, "input/day_22_0.txt", 5031); // 7 5 3
    }

    #[test]
    fn part_2_1() {
        test(&Day22::part_2, "input/day_22_1.txt", 74288);
    }

    fn test(f: &dyn Fn(&str) -> i64, path: &str, r: i64) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}