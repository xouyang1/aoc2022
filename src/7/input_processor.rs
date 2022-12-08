use lib::input_parser;

use std::collections::HashMap;
use std::str::FromStr;

pub type InputElementType = Dir;
pub type InputType = FileSystem;

pub fn get_input(file: &str) -> InputType {
    let raw = input_parser::read_file_to_string(file!(), file);
    FileSystem::build(&raw)
}

#[derive(Debug)]
pub struct Dir {
    name: String,
    pub total_size: u32,
    dirs: HashMap<String, usize>,
    parent: usize,
}

#[derive(Debug)]
enum Action {
    ChangeDir(String),
    ListDir,
    UpdateSize(u32),
    AddSubdir(String),
}

impl FromStr for Action {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.trim().split_whitespace().collect();
        match s[0] {
            "$" => match s[1] {
                "cd" => Ok(Action::ChangeDir(s[2].to_string())),
                "ls" => Ok(Action::ListDir),
                &_ => Err(()),
            },
            "dir" => Ok(Action::AddSubdir(s[1].to_string())),
            n_str => Ok(Action::UpdateSize(n_str.parse().unwrap())),
        }
    }
}

pub struct FileSystem {
    pub dirs: Vec<Dir>,
}

impl FileSystem {
    fn build(input: &str) -> Self {
        let mut dirs = Self::build_root("/");
        let mut current = 0;
        for line in input.lines() {
            Self::build_dirs(&mut dirs, &mut current, line)
        }
        const CHANGE_TO_PARENT_DIRECTORY: &str = "$ cd ..";
        while current != 0 {
            Self::build_dirs(&mut dirs, &mut current, CHANGE_TO_PARENT_DIRECTORY);
        }
        Self { dirs }
    }

    fn build_root(name: &str) -> Vec<Dir> {
        let mut dirs = Vec::new();
        dirs.push(Dir {
            name: "".to_string(),
            total_size: 0,
            dirs: HashMap::from([(name.to_string(), 1)]),
            parent: 0,
        });
        dirs.push(Dir {
            name: name.to_string(),
            total_size: 0,
            dirs: HashMap::new(),
            parent: 0,
        });
        dirs
    }

    fn build_dirs<'a>(dirs: &'a mut Vec<Dir>, current: &'a mut usize, line: &str) {
        match line.parse::<Action>().unwrap() {
            Action::ChangeDir(dir) => match dir.as_str() {
                ".." => {
                    let additional_size = dirs[*current].total_size;
                    let parent = dirs[*current].parent;
                    dirs[parent].total_size += additional_size;
                    *current = parent;
                }
                name => {
                    *current = *dirs[*current].dirs.get(name).unwrap();
                }
            },
            Action::ListDir => (),
            Action::UpdateSize(size) => dirs[*current].total_size += size,
            Action::AddSubdir(name) => {
                dirs.push(Dir {
                    name: name.clone(),
                    total_size: 0,
                    dirs: HashMap::new(),
                    parent: *current,
                });
                let i = dirs.len() - 1;
                dirs[*current].dirs.insert(name.clone(), i);
            }
        }
    }
}
