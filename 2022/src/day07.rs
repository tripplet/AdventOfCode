use std::{cell::RefCell, rc::Rc};

type NUMBER = u32;
type FileTree<'a> = Rc<RefCell<FSEntry<'a>>>;

#[derive(Debug)]
pub enum FSEntry<'a> {
    File {
        name: &'a str,
        size: NUMBER,
    },
    Directory {
        name: &'a str,
        entries: Vec<Rc<RefCell<FSEntry<'a>>>>,
    },
}

impl FSEntry<'_> {
    fn name(&self) -> &str {
        match self {
            FSEntry::File { name, size: _ } => name,
            FSEntry::Directory { name, entries: _ } => name,
        }
    }

    fn size(&self) -> NUMBER {
        match self {
            FSEntry::File { name: _, size } => *size,
            FSEntry::Directory { name: _, entries } => entries.iter().map(|entry| entry.as_ref().borrow().size()).sum(),
        }
    }
}

pub fn parse_input(input: &str) -> FileTree {
    let root = Rc::new(RefCell::new(FSEntry::Directory {
        name: "/",
        entries: vec![],
    }));

    let mut current = Rc::clone(&root);
    let mut cwd = vec![current.clone()];

    for line in input.trim().lines().skip(1) {
        let line = line.trim();

        if line == "$ ls" {
            // Ignore
            continue;
        } else if line == "$ cd .." {
            // Go up one directory
            cwd.pop();
            current = Rc::clone(cwd.last().unwrap());
        } else if line.starts_with("$ cd ") {
            // Go down one directory
            let name = &line[5..];
            let found4;

            {
                let current2 = &*current.as_ref().borrow();

                match current2 {
                    FSEntry::Directory { name: _, entries } => {
                        let found2 = entries.iter().find(|entry| {
                            let entry = &*entry.as_ref().borrow();
                            match entry {
                                FSEntry::Directory {
                                    name: entry_name,
                                    entries: _,
                                } => *entry_name == name,
                                FSEntry::File { name: _, size: _ } => false,
                            }
                        });

                        if let Some(found3) = found2 {
                            found4 = found3.clone();
                        } else {
                            panic!("unable to find directory: {}", name);
                        }
                    }
                    _ => unreachable!("current should never be a file"),
                }
            }

            current = found4;

            cwd.push(current.clone());
        } else {
            let current = &mut *current.as_ref().borrow_mut();

            match current {
                FSEntry::Directory {
                    name: _,
                    ref mut entries,
                } => {
                    if line.starts_with("dir") {
                        // Add directory to current directory
                        entries.push(Rc::new(RefCell::new(FSEntry::Directory {
                            name: &line[4..],
                            entries: vec![],
                        })));
                    } else {
                        // Add file to current directory
                        let parts = line
                            .split_once(" ")
                            .ok_or_else(|| format!("unable to parse file: {}", line))
                            .unwrap();
                        entries.push(
                            Rc::new(RefCell::new(FSEntry::File {
                                name: parts.1,
                                size: parts.0.parse().unwrap(),
                            }))
                            .into(),
                        );
                    }
                }
                _ => unreachable!("current should never be a file"),
            }
        }
    }

    root
}

pub fn part1(input: &FileTree) -> NUMBER {
    let mut result = 0;

    let entry = &*input.as_ref().borrow();
    match entry {
        FSEntry::Directory { name: _, entries } => {
            let directory_size = entry.size();

            if directory_size <= 100_000 {
                result += directory_size;
            }

            for sub_entry in entries {
                result += part1(sub_entry);
            }
        }
        _ => {}
    }

    result
}

pub fn part2(input: &FileTree) -> NUMBER {
    let root = &*input.as_ref().borrow();

    const TOTAL_DISK_SPACE: NUMBER = 70_000_000;
    const REQUIRED_FREE_DISK_SPACE_FOR_UPDATE: NUMBER = 30_000_000;

    // Get the disk usage
    let disk_usage = root.size();

    // Calculate the amount of free disk space required
    let required_free_disk_space = REQUIRED_FREE_DISK_SPACE_FOR_UPDATE - (TOTAL_DISK_SPACE - disk_usage);

    // Find the smallest directory must be deleted to free up enough space
    *all_direcory_sizes(input)
        .iter()
        .filter(|&&size| size >= required_free_disk_space)
        .min()
        .unwrap()
}

fn all_direcory_sizes<'a>(entry: &'a FileTree) -> Vec<NUMBER> {
    let mut result = vec![];

    let entry = &*entry.as_ref().borrow();
    match entry {
        FSEntry::Directory { name: _, entries } => {
            let directory_size = entry.size();

            result.push(directory_size);

            for sub_entry in entries {
                result.extend(all_direcory_sizes(sub_entry));
            }
        }
        _ => {}
    }

    result
}
