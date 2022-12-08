struct Day08;

impl Day08 {
    fn convert(s: &str) -> Vec<Vec<u8>> {
        s.lines().map(
            |line| line.as_bytes().iter().map(
                |tree| tree - b'0'
            ).collect()
        ).collect()
    }

    fn is_visible(x: usize, y: usize, w: usize, h: usize, trees: &Vec<Vec<u8>>) -> i32 {
        let tree = trees[y][x];

        let l = trees[y][0..x].iter().any(|t| tree <= *t);
        let r = trees[y][x + 1..w].iter().any(|t| tree <= *t);
        let t = trees[0..y].iter().any(|t| tree <= t[x]);
        let b = trees[y + 1..h].iter().any(|t| tree <= t[x]);

        let invisible = l && r && t && b;
        if invisible { 0 } else { 1 }
    }

    pub fn part_1(s: &str) -> i32 {
        let trees = Self::convert(s);
        let h = trees.len();
        let w = trees[0].len();

        let mut count = 0;
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let is_visible = Self::is_visible(x, y, w, h, &trees);
                count += is_visible;
            }
        }

        count + 2 * (h + w - 2) as i32
    }

    fn get_score(x: usize, y: usize, w: usize, h: usize, trees: &Vec<Vec<u8>>) -> i32 {
        let tree = trees[y][x];
        let mut score = 1;
        score *= (x - trees[y][0..x].iter().rposition(|&t| tree <= t).unwrap_or(0)) as i32;
        score *= (y - trees[0..y].iter().rposition(|t| tree <= t[x]).unwrap_or(0)) as i32;
        score *= (trees[y][x + 1..w].iter().position(|&t| tree <= t).unwrap_or(w - x - 2) + 1) as i32;
        score *= (trees[y + 1..h].iter().position(|t| tree <= t[x]).unwrap_or(h - y - 2) + 1) as i32;
        score
    }

    pub fn part_2(s: &str) -> i32 {
        let trees = Self::convert(s);
        let h = trees.len();
        let w = trees[0].len();

        let mut max = 0;
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                max = max.max(Self::get_score(x, y, w, h, &trees));
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day08::part_1, "input/day_08_0.txt", 21);
    }

    #[test]
    fn part_1_1() {
        test(&Day08::part_1, "input/day_08_1.txt", 1713);
    }

    #[test]
    fn part_2_0() {
        test(&Day08::part_2, "input/day_08_0.txt", 8);
    }

    #[test]
    fn part_2_1() {
        test(&Day08::part_2, "input/day_08_1.txt", 268464);
    }

    fn test(f: &dyn Fn(&str) -> i32, path: &str, r: i32) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}
