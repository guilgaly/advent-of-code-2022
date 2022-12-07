use common::itertools::Itertools;
use maplit::hashmap;
use std::collections::HashMap;
use std::error::Error;
use std::iter::once;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let fs_tree = build_fs_tree(INPUT)?;

    println!("Part 1 result: {}", part_1(&fs_tree));
    println!("Part 2 result: {}", part_2(&fs_tree));

    Ok(())
}

#[derive(Debug, PartialEq)]
enum FsNode {
    Dir(HashMap<String, FsNode>),
    File(u64),
}

fn part_1(fs_tree: &HashMap<String, FsNode>) -> u64 {
    let (_, all_dirs) = size_of_dir(&fs_tree);
    all_dirs.into_iter().filter(|d| *d < 100000).sum::<u64>()
}

fn part_2(fs_tree: &HashMap<String, FsNode>) -> u64 {
    let (total, all_dirs) = size_of_dir(&fs_tree);
    let free = 70000000 - total;
    let must_be_freed = 30000000 - free;
    all_dirs
        .into_iter()
        .filter(|d| *d >= must_be_freed)
        .sorted()
        .next()
        .unwrap()
}

fn size_of_dir(dir: &HashMap<String, FsNode>) -> (u64, Vec<u64>) {
    let mut dirs: Vec<(u64, Vec<u64>)> = vec![];
    let mut files: Vec<u64> = vec![];
    for n in dir.values() {
        match n {
            FsNode::Dir(inside_node) => dirs.push(size_of_dir(inside_node)),
            FsNode::File(s) => files.push(*s),
        }
    }
    let curr_dir_size =
        files.into_iter().sum::<u64>() + dirs.iter().map(|(size, _)| size).sum::<u64>();
    let inner_dirs = dirs
        .into_iter()
        .flat_map(|(d, inners)| once(d).chain(inners.into_iter()))
        .collect_vec();
    (curr_dir_size, inner_dirs)
}

fn build_fs_tree(input: &str) -> Result<HashMap<String, FsNode>, String> {
    let mut current_path: Vec<String> = vec![];
    let mut fs: HashMap<String, FsNode> = HashMap::new();

    for line in input.lines() {
        if line.starts_with("$ ls") {
            // Do nothing
        } else if line.starts_with("$ cd ") {
            match line.split_at(5).1.trim() {
                "/" => current_path = vec![],
                ".." => {
                    current_path.pop();
                }
                path => current_path.push(path.to_owned()),
            }
        } else if line.starts_with("$") {
            Err(format!("Invalid: '{}'", line))?;
        } else {
            let mut parts = line.split(" ");
            let res_type = parts.next().ok_or(format!("Invalid: '{}'", line))?;
            let name = parts
                .next()
                .ok_or(format!("Invalid: '{}'", line))?
                .to_owned();
            if res_type == "dir" {
                add_node(
                    &mut fs,
                    &current_path,
                    name.clone(),
                    FsNode::Dir(HashMap::new()),
                );
            } else {
                let size = res_type
                    .parse::<u64>()
                    .map_err(|_| format!("Invalid: '{}'", line))?;
                add_node(&mut fs, &current_path, name.clone(), FsNode::File(size));
            }
        }
    }
    Ok(fs)
}

fn add_node(fs: &mut HashMap<String, FsNode>, path: &[String], node_name: String, node: FsNode) {
    let mut curr_dir = fs;
    for p in path {
        match curr_dir.get_mut(p) {
            None => {
                panic!("Directory not yet visited");
            }
            Some(FsNode::Dir(d)) => {
                curr_dir = d;
            }
            Some(FsNode::File(_)) => {
                panic!("Cannot have a file in path");
            }
        }
    }
    curr_dir.insert(node_name, node);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fs_tree() -> HashMap<String, FsNode> {
        let test_input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        build_fs_tree(test_input).unwrap()
    }

    #[test]
    fn test_build_fs_tree() {
        let expected: HashMap<String, FsNode> = hashmap! {
            "a".to_owned() => FsNode::Dir(hashmap! {
                "e".to_owned() => FsNode::Dir(hashmap! {
                    "i".to_owned() => FsNode::File(584)
                }),
                "f".to_owned() => FsNode::File(29116),
                "g".to_owned() => FsNode::File(2557),
                "h.lst".to_owned() => FsNode::File(62596),
            }),
            "b.txt".to_owned() => FsNode::File(14848514),
            "c.dat".to_owned() => FsNode::File(8504156),
            "d".to_owned() => FsNode::Dir(hashmap! {
                "j".to_owned() => FsNode::File(4060174),
                "d.log".to_owned() => FsNode::File(8033020),
                "d.ext".to_owned() => FsNode::File(5626152),
                "k".to_owned() => FsNode::File(7214296),
            })
        };
        assert_eq!(fs_tree(), expected);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&fs_tree()), 95437);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&fs_tree()), 24933642);
    }
}
