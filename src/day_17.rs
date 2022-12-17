use std::collections::{HashMap, VecDeque};

struct Day17;

impl Day17 {
    /*
    fn print_cup_with_figure(cup: &VecDeque<u8>, figure: &Vec<u8>, (px, py): (i32, usize)) {
        for y in (1..cup.len()).rev() {
            print!("|");
            for x in 0..7 {
                let line = {
                    if py <= y && y < py + figure.len() {
                        if px < 0 { figure[y - py] >> -px }
                        else { figure[y - py] << px }
                    } else {
                        0
                    }
                };
                if cup[y] & 1 << x != 0 {
                    print!("#")
                } else if line & 1 << x != 0 {
                    print!("@")
                } else {
                    print!(".")
                }
            }
            println!("|");
        }
        println!("+-------+");
        println!();
    }
    */

    /*
    fn print_cup(cup: &VecDeque<u8>) {
        for line in cup.iter().skip(1).rev() {
            print!("|");
            for i in 0..7 {
                if line & 1 << i != 0 {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!("|");
        }
        println!("+-------+");
        println!();
    }
    */

    pub fn solution(s: &str, count: usize) -> usize {
        fn find_top(cup: &VecDeque<u8>) -> usize {
            cup.iter().rposition(|&line| 0 < line).unwrap()
        }

        fn test_figure(cup: &VecDeque<u8>, figure: &Vec<u8>, (px, py): (i8, usize)) -> bool {
            for y in 0..figure.len() {
                let line_c = cup[y + py];
                let line_f = figure[y];
                let line_f = if px < 0 { line_f >> -px }
                                      else { line_f << px };
                if line_c & line_f != 0 {
                    return false
                }
            }
            true
        }

        fn place_figure(cup: &mut VecDeque<u8>, figure: &Vec<u8>, (px, py): (i8, usize)) {
            for y in 0..figure.len() {
                cup[y + py] |= if px < 0 { figure[y] >> -px }
                                    else { figure[y] << px };
            }
        }

        let input: Vec<i8> = s.as_bytes().iter().map(|&b| if b == b'>' { 1 } else { -1 }).collect();
        let figures = {
            [
                (4, vec![0b1111]),
                (3, vec![
                    0b010,
                    0b111,
                    0b010,
                ]),
                (3, vec![
                    0b111,
                    0b100,
                    0b100,
                ]),
                (1, vec![
                    1,
                    1,
                    1,
                    1,
                ]),
                (2, vec![
                    0b11,
                    0b11,
                ]),
            ]
        };

        let mut cup = VecDeque::with_capacity(1024);
        cup.push_back(0b0111_1111);

        let mut try_skip = true;
        let mut history = HashMap::new();
        let mut cup_history = 0_u128;

        let mut input_i = 0;
        let mut height = 0;
        let mut iteration = 0;
        while iteration < count {
            let figure_i = iteration % figures.len();
            let figure = &figures[figure_i];
            let (figure_width, figure) = (figure.0, &figure.1);

            let top = find_top(&cup);
            // println!("top={top}");
            let figure_height = figure.len();
            let spawn_line = top + 3 + figure_height;
            while cup.len() <= spawn_line {
                cup.push_back(0);
            }

            // spawn
            let mut position = (2, spawn_line - figure_height + 1);
            // Self::print_cup_with_figure(&cup, figure, position);

            let mut x = position.0;
            for _ in 0..4 {
                let d = input[input_i];
                input_i = (input_i + 1) % input.len();

                let new_x = x + d;
                if 0 <= new_x && new_x + figure_width < 8 {
                    x = new_x
                }
            }
            position.0 = x;
            position.1 -= 3;
            // Self::print_cup_with_figure(&cup, figure, position);

            loop {
                let can_move = if 0 < position.1 {
                    let new_position = (position.0, position.1 - 1);
                    if test_figure(&cup, figure, new_position) {
                        position = new_position;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };

                if !can_move {
                    place_figure(&mut cup, figure, position);
                    cup_history = (cup_history << 8) | cup[position.1 - 1] as u128;
                    if try_skip && 16 <= iteration {
                        let k = (input_i, figure_i as u8, position.0, cup_history);
                        if let Some((i, h)) = history.get(&k) {
                            let d = iteration - i;
                            let hh = find_top(&cup);
                            height = (hh - h) * ((count - i) / d - 1);
                            iteration = count - (count - i) % d;
                            try_skip = false;
                        } else {
                            history.insert(k, (iteration, find_top(&cup)));
                        }
                    }
                    break;
                }

                let d = input[input_i];
                input_i = (input_i + 1) % input.len();

                let new_position = (position.0 + d, position.1);
                if 0 <= new_position.0 && new_position.0 + figure_width < 8 {
                    let can_move = test_figure(&cup, &figures[figure_i].1, new_position);
                    if can_move { position = new_position }
                }
                // print_cup_with_figure(&cup, &figures[figure_i].1, position);
            }

            iteration += 1;
        }
        // print_cup(&cup);

        height + find_top(&cup)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day17::solution, "input/day_17_0.txt", 2022, 3068);
    }

    #[test]
    fn part_1_1() {
        test(&Day17::solution, "input/day_17_1.txt", 2022, 3067);
    }

    #[test]
    fn part_2_0() {
        test(&Day17::solution, "input/day_17_0.txt", 1_000_000_000_000, 1514285714288);
    }

    #[test]
    fn part_2_1() {
        test(&Day17::solution, "input/day_17_1.txt", 1_000_000_000_000, 1514369501484);
    }

    fn test(f: &dyn Fn(&str, usize) -> usize, path: &str, count: usize, r: usize) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input, count), r);
    }
}
