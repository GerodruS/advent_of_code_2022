use std::collections::HashMap;
use std::string::ToString;

struct Day07;

impl Day07 {
    fn get_tree(s: &str) -> HashMap<String, Folder> {
        let mut folders = HashMap::new();
        folders.insert("/".to_string(), Folder::new());
        let mut path = "/".to_string();
        let mut temp = String::new();
        for line in s.lines().skip(1) {
            if line.starts_with("$ cd") {
                let dir = line.split(' ').nth(2).unwrap();
                if dir == ".." {
                    path.pop();
                    while path.chars().last().unwrap() != '/' { path.pop(); }
                } else {
                    path.push_str(dir);
                    path.push('/');
                }
            } else if line.starts_with("$ ls") {
                //
            } else if line.starts_with('d') {
                let dir = line.split(' ').nth(1).unwrap();
                temp.push_str(&path);
                temp.push_str(dir);
                temp.push('/');
                if !folders.contains_key(&temp) {
                    folders.insert(temp.clone(), Folder::new());
                }
                folders.get_mut(&path).unwrap().folders.push(temp.clone());
                temp.clear();
            } else {
                let mut split = line.split(' ');
                let size = split.next().unwrap().parse::<usize>().unwrap();
                let file = split.next().unwrap();
                temp.push_str(&path);
                temp.push_str(file);
                folders.get_mut(&path).unwrap().files.push(File {
                    // name: file.to_string(),
                    size,
                });
                temp.clear();
            }
        }

        // Self::print_tree(&folders);
        folders
    }

    // fn print_tree(folders: &HashMap<String, Folder>) {
    //     for elem in folders {
    //         println!("{} {}", elem.0, elem.1.size);
    //         for file in &elem.1.files {
    //             println!("  file: {} {}", file.name, file.size);
    //         }
    //         for folder in &elem.1.folders {
    //             println!("  fldr: {}", folder);
    //         }
    //         println!();
    //     }
    // }

    fn get_sizes(folders: &HashMap<String, Folder>) -> HashMap<&String, usize> {
        fn get_size<'a>(folder: &'a String, folders: &'a HashMap<String, Folder>, sizes: &mut HashMap<&'a String, usize>) -> usize {
            if let Some(s) = sizes.get(folder) { return *s }
            let folder_data = folders.get(folder).unwrap();
            let mut s = 0;
            for file in &folder_data.files {
                s += file.size;
            }
            for sub_folder in &folder_data.folders {
                s += get_size(sub_folder, folders, sizes);
            }
            sizes.insert(folder, s);
            s
        }

        let mut sizes = HashMap::new();
        for folder in folders.keys() {
            get_size(folder, folders, &mut sizes);
        }
        sizes
    }

    pub fn part_1(s: &str) -> i32 {
        let folders = Self::get_tree(s);
        let sizes = Self::get_sizes(&folders);

        let mut r = 0;
        for (_, s) in sizes {
            r += if s <= 100000 {
                s
            } else {
                0
            };
        }

        r as i32
    }

    pub fn part_2(s: &str) -> i32 {
        let folders = Self::get_tree(s);
        let sizes = Self::get_sizes(&folders);

        let delta_size = sizes.get(&"/".to_string()).unwrap() + 30_000_000 - 70_000_000;

        let mut r = usize::MAX;
        for (_, s) in sizes {
            if delta_size <= s {
                r = r.min(s);
            }
        }
        r as i32
    }
}

struct Folder {
    folders: Vec<String>,
    files: Vec<File>,
}

impl Folder {
    fn new() -> Self {
        Folder {
            folders: Vec::new(),
            files: Vec::new(),
        }
    }
}

struct File {
    // name: String,
    size: usize,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_1_0() {
        test(&Day07::part_1, "input/day_07_0.txt", 95437);
    }

    #[test]
    fn part_1_1() {
        test(&Day07::part_1, "input/day_07_1.txt", 1243729);
    }

    #[test]
    fn part_2_0() {
        test(&Day07::part_2, "input/day_07_0.txt", 24933642);
    }

    #[test]
    fn part_2_1() {
        test(&Day07::part_2, "input/day_07_1.txt", 4443914);
    }

    fn test(f: &dyn Fn(&str) -> i32, path: &str, r: i32) {
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(f(&input), r);
    }
}


