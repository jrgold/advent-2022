use std::collections::HashMap;

#[derive(Debug)]
enum FileInfo {
    Directory(String),
    File(String, i32),
}

#[derive(Debug)]
enum Command {
    Ls(Vec<FileInfo>),
    CdRoot,
    CdUp,
    Cd(String),
}

#[derive(Debug)]
enum FileSystem {
    Directory(HashMap<String, FileSystem>),
    File(i32),
}

fn output_from_file(path: &str) -> Vec<Command> {
    let input = std::fs::read_to_string(path).unwrap();
    input.lines().fold(Vec::new(), |mut list, line| {
        if line == "$ ls" {
            list.push(Command::Ls(vec![]));
        } else if line == "$ cd /" {
            list.push(Command::CdRoot);
        } else if line == "$ cd .." {
            list.push(Command::CdUp);
        } else if line.starts_with("$ cd") {
            list.push(Command::Cd(line[5..].to_owned()))
        } else {
            let Command::Ls(mut files) = list.pop().unwrap() else { unreachable!(); };
            let info = if line.starts_with("dir ") {
                FileInfo::Directory(line[4..].to_owned())
            } else {
                let (size, name) = line.split_once(" ").unwrap();
                FileInfo::File(name.to_owned(), size.parse().unwrap())
            };
            files.push(info);
            list.push(Command::Ls(files));
        }

        list
    })
}

fn merge_ls_into(mut fs: &mut FileSystem, path: &[String], new: Vec<(String, FileSystem)>) {
    for dir in path {
        let FileSystem::Directory(f) = fs else { unreachable!() };
        fs = f.get_mut(dir).unwrap();
    }

    let FileSystem::Directory(target_dir) = fs else { unreachable!(); };

    for (name, new_fs) in new.into_iter() {
        target_dir.entry(name).and_modify(|_| ()).or_insert(new_fs);
    }
}

fn filesystem_from_output(commands: Vec<Command>) -> FileSystem {
    let mut path: Vec<String> = vec![];

    let mut fs = FileSystem::Directory(HashMap::new());

    for command in commands.into_iter() {
        match command {
            Command::CdRoot => { path.drain(0..); },
            Command::CdUp => { path.pop(); },
            Command::Cd(dir_name) => { path.push(dir_name); },
            Command::Ls(infos) => {
                let contents: Vec<(String, FileSystem)> = infos.into_iter().map(|info| {
                    match info {
                        FileInfo::Directory(name) => (name, FileSystem::Directory(HashMap::new())),
                        FileInfo::File(name, size) => (name, FileSystem::File(size))
                    }
                }).collect();

                merge_ls_into(&mut fs, &path, contents);
            }
        }
    }

    fs
}

fn all_directory_sizes_rec(sizes: &mut HashMap<Vec<String>, i32>, path: &mut Vec<String>, dir: HashMap<String, FileSystem>) -> i32 {
    let mut total = 0;
    for (name, file) in dir {
        match file {
            FileSystem::Directory(subdir) => {
                path.push(name);
                total += all_directory_sizes_rec(sizes, path, subdir);
                path.pop();
            },
            FileSystem::File(size) => { total += size; },
        }
    }
    sizes.insert(path.clone(), total);
    total
}

fn all_directory_sizes(fs: FileSystem) -> HashMap<Vec<String>, i32> {
    let mut sizes = HashMap::new();
    let mut path = vec![];
    let FileSystem::Directory(dir) = fs else { unreachable!(); };
    all_directory_sizes_rec(&mut sizes, &mut path, dir);
    sizes
}

#[allow(dead_code)]
pub fn part_1() -> i32 {
    let input = output_from_file("input/real/7.txt");
    let fs = filesystem_from_output(input);
    let sizes = all_directory_sizes(fs);
    sizes.values().filter(|&&size| size <= 100_000).sum()
}

#[allow(dead_code)]
pub fn part_2() -> i32 {
    let input = output_from_file("input/real/7.txt");
    let fs = filesystem_from_output(input);
    let sizes = all_directory_sizes(fs);
    let unused_space = 70_000_000 - sizes[&vec![]];
    *sizes.values().filter(|&&size| size >= 30_000_000 - unused_space).min().unwrap()
}
