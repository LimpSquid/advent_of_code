use crate::utils;
use std::io::Read;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

impl File {
    fn new(name: &str, size: u32) -> File {
        File {
            name: name.into(),
            size,
        }
    }
}

type DirectoryRef = Rc<RefCell<Directory>>;

#[derive(Debug)]
struct Directory {
    name: String,
    dirs: Vec<DirectoryRef>,
    files: Vec<File>,
}

impl Directory {
    fn new(name: &str) -> Directory {
        Directory {
            name: name.into(),
            dirs: Vec::new(),
            files: Vec::new(),
        }
    }

    fn to_ref(self) -> DirectoryRef {
        Rc::new(RefCell::new(self))
    }

    fn get_or_insert_dir(&mut self, dir: Directory) -> DirectoryRef {
        let x = self.dirs.iter()
            .find(|x| x.as_ref().borrow().name == dir.name);

        match x {
            Some(x) => x.clone(),
            None => {
                let dir = dir.to_ref();
                self.dirs.push(dir.clone());
                dir
            }
        }
    }

    fn set_dirs(&mut self, dirs: Vec<Directory>) {
        self.dirs.clear();
        dirs.into_iter().for_each(|dir| self.dirs.push(dir.to_ref()));
    }

    fn set_files(&mut self, files: Vec<File>) {
        self.files = files;
    }

    fn size(&self) -> u32 {
        let dir_sizes: u32 = self.dirs.iter()
            .map(|x| x.as_ref().borrow().size())
            .sum();
        let file_sizes: u32 = self.files.iter()
            .map(|x| x.size)
            .sum();

        dir_sizes + file_sizes
    }

    fn dirs(&self) -> Vec<DirectoryRef> {
        let mut flattened:Vec<DirectoryRef> = Vec::new();
        self.dirs.iter().for_each(|dir| {
            let mut dirs = dir.as_ref().borrow().dirs();
            flattened.append(&mut dirs);
            flattened.push(dir.clone());
        });
        flattened
    }
}

#[derive(Debug)]
enum Command {
    DirRoot,
    DirPush(Directory),
    DirPop,
    List(Vec<Directory>, Vec<File>),
}

impl Command {
    fn parse(str: &str) -> Result<Command, String> {
        let lines: Vec<&str> = str.split('\n').collect();
        let command: Vec<&str> = lines[0].split(' ').collect();
        match command[..] {
            // cd /, cd .., cd <file_name>
            ["cd", dir] => match dir {
                "/"  => Ok(Command::DirRoot),
                ".." => Ok(Command::DirPop),
                name => Ok(Command::DirPush(Directory::new(name)))
            },
            // ls, no dirs or files
            ["ls"] if lines.len() == 1 => Ok(Command::List(Vec::new(), Vec::new())),
            // ls, atleast one dir or file
            ["ls"] => {
                let mut dirs: Vec<Directory> = Vec::new();
                let mut files: Vec<File> = Vec::new();
                lines[1..].into_iter().for_each(|line| {
                    let split: Vec<&str> = line.split(' ').collect();
                    match split[..] {
                        ["dir", name] => { dirs.push(Directory::new(name)); }
                        [size, name] => {
                            if let Ok(size) = size.parse::<u32>() {
                                files.push(File::new(name, size));
                            }
                        }
                        _ => {}
                    }
                });
                Ok(Command::List(dirs, files))
            }
            _ => Err(format!("Oops, I don't understand this command: '{}'", str))
        }
    }
}

pub fn solve(files_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = utils::input_file_reader(files_dir)?;
    let mut lines = String::new();
    reader.read_to_string(&mut lines)?;

    let fs = Directory::new("/").to_ref();
    let mut stack: Vec<DirectoryRef> = vec![fs.clone()];

    lines.split("$")
        .filter_map(|line| Command::parse(line.trim()).ok())
        .for_each(|cmd| match cmd {
            Command::DirRoot => stack.truncate(1),
            Command::DirPush(dir) => {
                let dir = stack.last()
                    .unwrap()
                    .borrow_mut()
                    .get_or_insert_dir(dir);
                stack.push(dir);
            }
            Command::DirPop => { stack.pop(); }
            Command::List(dirs, files) => {
                let mut curr = stack.last().unwrap().borrow_mut();
                curr.set_dirs(dirs);
                curr.set_files(files);
            }
        });

    part_one(fs.clone())?;
    part_two(fs.clone())?;
    Ok(())
}

fn part_one(fs: DirectoryRef) -> Result<(), Box<dyn std::error::Error>> {
    let fs = fs.as_ref().borrow();
    let answer: u32 = fs.dirs()
        .into_iter()
        .filter_map(|dir| {
            let size = dir.as_ref().borrow().size();
            if size < 100000 { Some(size) } else { None }
        })
        .sum();

    println!("Part one answer is: {}", answer);
    Ok(())
}

fn part_two(fs: DirectoryRef) -> Result<(), Box<dyn std::error::Error>> {
    let fs = fs.as_ref().borrow();
    let space_needed = 30000000 - (70000000 - fs.size());

    let answer: u32 = fs.dirs()
        .into_iter()
        .filter_map(|dir| {
            let size = dir.as_ref().borrow().size();
            if size >= space_needed { Some(size) } else { None }
        })
        .min().unwrap();

    println!("Part two answer is: {}", answer);
    Ok(())
}
