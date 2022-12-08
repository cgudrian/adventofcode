use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Dir {
    dirs: HashMap<String, Dir>,
    files: HashMap<String, usize>,
}

impl Dir {
    fn new() -> Dir {
        Dir {
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }
}

type Path = Vec<String>;

fn get_dir<'a>(root: &'a mut Dir, path: &'a Path) -> &'a mut Dir {
    path.iter().fold(root, |dir, subdir| {
        dir.dirs.entry(subdir.clone()).or_insert_with(Dir::new)
    })
}

fn get_dir_sizes(dir: &Dir, sizes: &mut Vec<usize>) -> usize {
    let subdir_size = dir
        .dirs
        .iter()
        .fold(0, |size, (_, dir)| size + get_dir_sizes(dir, sizes));
    let file_size = dir.files.iter().fold(0, |size, (_, s)| size + s);

    let total_size = subdir_size + file_size;
    sizes.push(total_size);
    total_size
}

fn main() {
    let mut filesystem = Dir::new();

    {
        let input = fs::read("../input.txt").unwrap();
        let input = std::str::from_utf8(&input).unwrap();
        let lines = input
            .lines()
            .map(|line| -> Vec<&str> { line.split(" ").collect() });
        let mut path: Path = Vec::new();
        lines.fold(
            (&mut filesystem, &mut path),
            |(filesystem, path), cmd| match cmd.as_slice() {
                ["$", "cd", dir] => match *dir {
                    "/" => {
                        path.clear();
                        (filesystem, path)
                    }

                    ".." => {
                        path.pop();
                        (filesystem, path)
                    }

                    dir => {
                        path.push(dir.into());
                        (filesystem, path)
                    }
                },

                [size, filename] if size.parse::<usize>().is_ok() => {
                    let cwd = get_dir(filesystem, path);
                    cwd.files.insert((*filename).into(), size.parse().unwrap());
                    (filesystem, path)
                }

                _ => (filesystem, path),
            },
        );
    }

    let mut sizes = Vec::new();
    let filesystem_size = get_dir_sizes(&filesystem, &mut sizes);

    let answer1: usize = sizes.iter().filter(|s| **s <= 100000).sum();
    println!("Answer 1: {answer1}");

    let filesystem_capacity = 70000000;
    let space_required = 30000000;
    let currently_free = filesystem_capacity - filesystem_size;
    let minimum_space_to_free = space_required - currently_free;

    let answer2 = sizes.iter().filter(|s| **s >= minimum_space_to_free).min().unwrap();
    println!("Answer 2: {answer2}");
}
