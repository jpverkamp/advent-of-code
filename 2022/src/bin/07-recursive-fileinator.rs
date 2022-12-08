use aoc::*;
use std::{borrow::Borrow, cell::RefCell, fmt::Display, path::Path, sync::Arc};

// Represents a 'thing' on a file system, either:
//  - A named directory which can contain directories or files
//  - A named file which has a size
#[derive(Clone)]
enum FileSystemThing {
    Directory {
        name: String,
        parent: Option<Arc<FileSystemThing>>,
        children: RefCell<Vec<Arc<FileSystemThing>>>,
    },
    File {
        name: String,
        size: usize,
    },
}

impl FileSystemThing {
    // Build a file system thing from an iter of commands run (cd and ls)
    fn from<I>(mut iter: I) -> Arc<Self>
    where
        I: Iterator<Item = String>,
    {
        use FileSystemThing::*;

        // Build a special :ROOT: node to start with
        let root = Arc::new(Directory {
            name: String::from(":ROOT:"),
            parent: None,
            children: RefCell::new(Vec::new()),
        });

        // Keep track of where we currently are after cds
        let current = RefCell::new(root.clone());

        while let Some(next) = iter.next() {
            let mut parts = next.split_ascii_whitespace();

            if next.starts_with("$ cd") {
                // cd changes directory and has three cases:
                //  "/" goes to root
                //  ".." goes up one level
                //  anything else goes into the named child directory

                let path = String::from(parts.last().expect("cd must have a directory"));

                // Build the next node depending on what we're cding too ("/", "..", or a name)
                let next = match current.borrow().as_ref() {
                    Directory { parent, .. } => {
                        if path == ".." {
                            match parent {
                                Some(parent) => parent.clone(),
                                _ => panic!("must have parent set to .."),
                            }
                        } else if path == "/" {
                            root.clone()
                        } else if let Some(child) = current.borrow().get(path) {
                            child
                        } else {
                            current.borrow().clone()
                        }
                    }
                    _ => current.borrow().clone(),
                };
                current.replace(next);
            } else if next.starts_with("$ ls") {
                // Starting an LS, nothing to do on this line
            } else if next.starts_with("dir") {
                // If we see a line starting with dir we're in an ls, create the directory

                let name = parts.last().expect("directory in ls must have a name");

                // If the file/directory already exists, we ran ls twice, ignore this
                if let Some(_child) = current.borrow().get(String::from(name)) {
                    continue;
                }

                // Build the new child directory referencing current as the parent
                let child = Arc::new(Directory {
                    name: String::from(name),
                    parent: Some(current.borrow().clone()),
                    children: RefCell::new(Vec::new()),
                });

                // Add a reference to the new directory to current's children
                match current.borrow().as_ref() {
                    Directory { children, .. } => {
                        children.borrow_mut().push(child);
                    }
                    _ => panic!("somehow tried to ls a file"),
                }
            } else {
                // Otherwise, it's a line containing the size and name of a file
                let size = parts
                    .next()
                    .expect("must have size")
                    .parse::<usize>()
                    .expect("size must be a usize");
                let name = parts.next().expect("must have a name");

                // If the file/directory already exists, we ran ls twice, ignore this
                if let Some(_child) = current.borrow().get(String::from(name)) {
                    continue;
                }

                // Build the new file, doesn't need a parent at least
                let child = Arc::new(File {
                    name: String::from(name),
                    size,
                });

                // Add a reference to the new file to current's children
                match current.borrow().as_ref() {
                    Directory { children, .. } => {
                        children.borrow_mut().push(child);
                    }
                    _ => panic!("somehow tried to put a file into another file"),
                }
            }
        }

        root.clone()
    }

    // Get a file or directory by name from the current node
    pub fn get(&self, key: String) -> Option<Arc<FileSystemThing>> {
        use FileSystemThing::*;

        match self {
            Directory { children, .. } => {
                for child in children.borrow().iter() {
                    match child.as_ref() {
                        Directory { name, .. } if *name == key => return Some(child.clone()),
                        File { name, .. } if *name == key => return Some(child.clone()),
                        _ => {} // Non-matching name
                    }
                }
            }
            _ => panic!("somehow tried to put a child of a file"),
        }

        None
    }

    // Get the size for a file or directory
    // Files directly have sizes, directories recursively sum their children's sizes
    pub fn size(&self) -> usize {
        use FileSystemThing::*;

        match self {
            Directory { children, .. } => {
                return children
                    .borrow()
                    .iter()
                    .map(|child| child.as_ref().size())
                    .sum()
            }
            File { size, .. } => *size,
        }
    }

    // Return an iterator over all nodes in the tree
    pub fn walk(&self) -> FileSystemIterator {
        FileSystemIterator::new(Arc::new(self.clone()))
    }

    // A function to dump out the tree structure without fields such as parent
    #[allow(dead_code)]
    pub fn dump(&self) {
        fn dump_indent(node: &FileSystemThing, level: usize) {
            use FileSystemThing::*;

            match node {
                Directory { name, children, .. } => {
                    print!("{}{}/ ({})\n", "  ".repeat(level), name, node.size());
                    for child in children.borrow().iter() {
                        dump_indent(&child, level + 1);
                    }
                }
                File { name, size, .. } => {
                    print!("{}{} ({})\n", "  ".repeat(level), name, size);
                }
            }
        }

        dump_indent(self, 0);
    }
}

// A function to display file system things with their size
impl Display for FileSystemThing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FileSystemThing::*;

        f.write_str(&match self {
            Directory { name, .. } => format!("{}/ ({})", name, self.size()),
            File { name, size } => format!("{} ({})", name, size),
        })
    }
}

// Iterate recursively over file systems
// Keep a stack of the children we've seen but not returned so far
struct FileSystemIterator {
    stack: Vec<Arc<FileSystemThing>>,
}

impl FileSystemIterator {
    fn new(root: Arc<FileSystemThing>) -> Self {
        FileSystemIterator { stack: vec![root] }
    }
}

impl Iterator for FileSystemIterator {
    type Item = Arc<FileSystemThing>;

    fn next(&mut self) -> Option<Self::Item> {
        use FileSystemThing::*;

        if self.stack.is_empty() {
            return None;
        }

        let next = self.stack.pop().unwrap();
        if let Directory { children, .. } = next.borrow() {
            for child in children.borrow().iter() {
                self.stack.push(child.clone());
            }
        }

        Some(next)
    }
}

fn part1(filename: &Path) -> String {
    let root = FileSystemThing::from(iter_lines(filename));

    let mut total_sizes = 0;
    for node in root.walk() {
        match node.borrow() {
            FileSystemThing::Directory { .. } => {
                let size = node.size();
                if size <= 100000 {
                    total_sizes += size;
                }
            }
            _ => {}
        }
    }

    total_sizes.to_string()
}

fn part2(filename: &Path) -> String {
    let root = FileSystemThing::from(iter_lines(filename));

    let total_disk = 70000000;
    let needed = 30000000;
    let used = root.size();
    let available = total_disk - used;
    let target_to_free = needed - available;

    // We need the smallest directory at least larger than target_to_free
    let mut freeable = usize::MAX;

    for node in root.walk() {
        match node.borrow() {
            FileSystemThing::Directory { .. } => {
                let size = node.size();
                if size > target_to_free && size < freeable {
                    freeable = size;
                }
            }
            _ => {}
        }
    }

    freeable.to_string()
}

fn main() {
    aoc_main(part1, part2);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use aoc::aoc_test;

    #[test]
    fn test1() {
        aoc_test("07", part1, "1453349")
    }

    #[test]
    fn test2() {
        aoc_test("07", part2, "2948823")
    }
}
