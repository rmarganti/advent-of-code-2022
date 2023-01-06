use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    if let Err(e) = do_main() {
        eprintln!("{}", e);
    }
}

fn do_main() -> shared::Result<()> {
    let content = shared::read_file_from_args()?;
    let commands = text_to_commands(content);
    let tree = Node::from_commands(&commands);

    let mut all_dir_sizes = tree.all_dir_sizes();
    let result: usize = all_dir_sizes.iter().filter(|item| **item <= 100000).sum();

    println!("Part1: {:#?}", result);

    all_dir_sizes.sort();

    const FILE_SYSTEM_SIZE: usize = 70_000_000;
    const NEEDED_SIZE: usize = 30_000_000;

    let used_size = tree.size();

    for dir_size in all_dir_sizes {
        if FILE_SYSTEM_SIZE - used_size + dir_size > NEEDED_SIZE {
            println!("Part2: {}", dir_size);
            break;
        }
    }

    Ok(())
}

// ------------------------------------------------
// Node
// ------------------------------------------------

type NodeHandle = Rc<RefCell<Node>>;

#[derive(Debug, Default)]
struct Node {
    size: usize,
    children: HashMap<String, NodeHandle>,
    parent: Option<NodeHandle>,
}

impl Node {
    pub fn from_commands(commands: &[Command]) -> Self {
        let root = Rc::new(RefCell::new(Node::default()));
        let mut node = root.clone();

        for command in commands {
            match command {
                Command::Cd(cd) => match cd.destination.as_str() {
                    "/" => {
                        node = root.clone();
                    }
                    ".." => {
                        let parent = node.borrow().parent.clone().unwrap();
                        node = parent;
                    }
                    _ => {
                        let child = node
                            .borrow_mut()
                            .children
                            .entry(cd.destination.to_owned())
                            .or_default()
                            .clone();

                        node = child;
                    }
                },

                Command::Ls(ls) => {
                    for directory in &ls.directories {
                        let directory = node
                            .borrow_mut()
                            .children
                            .entry(directory.to_owned())
                            .or_default()
                            .clone();

                        let mut directory = directory.borrow_mut();
                        directory.parent = Some(node.clone())
                    }

                    for (file_name, file_size) in &ls.files {
                        let file = node
                            .borrow_mut()
                            .children
                            .entry(file_name.to_owned())
                            .or_default()
                            .clone();

                        let mut file = file.borrow_mut();
                        file.size = *file_size;
                        file.parent = Some(node.clone());
                    }
                }
            }
        }

        root.take()
    }

    pub fn size(&self) -> usize {
        let mut size = self.size;

        for child in self.children.values() {
            size += child.borrow().size();
        }

        size
    }

    pub fn all_dir_sizes(&self) -> Vec<usize> {
        let mut sizes = vec![];

        let self_size = self.size();
        if self_size > 0 {
            sizes.push(self_size);
        }

        for child in self.children.values() {
            let child = child.borrow();
            let child_size = child.size;

            // Directories have size of 0.
            if child_size == 0 {
                let sub_dirs = child.all_dir_sizes();

                for sub_dir in sub_dirs {
                    sizes.push(sub_dir);
                }
            }
        }

        sizes
    }
}

// ------------------------------------------------
// Commands
// ------------------------------------------------

fn text_to_commands(content: String) -> Vec<Command> {
    let mut commands: Vec<Command> = vec![];
    let mut lines = content.lines();
    let mut buffer = String::new();

    while let Some(line) = lines.next() {
        if line.starts_with("$ ") && !buffer.is_empty() {
            let command = parse_single_command(&buffer);
            commands.push(command);

            buffer.clear();
        }

        buffer.push_str(line);
        buffer.push('\n');
    }

    if !buffer.is_empty() {
        let command = parse_single_command(&buffer);
        commands.push(command);
    }

    commands
}

fn parse_single_command(data: &str) -> Command {
    let mut lines = data.lines();

    let first_line = lines.next().unwrap();
    let cmd = &first_line[2..4];

    let cmd = match cmd {
        "cd" => Command::Cd(CdCommand::new(first_line[5..].to_string())),

        "ls" => {
            let mut ls_cmd = LsCommand::new();

            for line in lines {
                if line.starts_with("dir") {
                    ls_cmd.add_dir(line[4..].to_string());
                } else {
                    let pieces: Vec<&str> = line.split(' ').collect();
                    let size: usize = pieces[0].parse().unwrap();
                    ls_cmd.add_file(pieces[1].to_string(), size);
                }
            }

            Command::Ls(ls_cmd)
        }
        _ => panic!("Invalid command"),
    };

    cmd
}

#[derive(Debug)]
enum Command {
    Cd(CdCommand),
    Ls(LsCommand),
}

#[derive(Debug)]
struct CdCommand {
    destination: String,
}

impl CdCommand {
    fn new(destination: String) -> Self {
        CdCommand { destination }
    }
}

#[derive(Debug)]
struct LsCommand {
    directories: Vec<String>,
    files: HashMap<String, usize>,
}

impl LsCommand {
    pub fn new() -> Self {
        LsCommand {
            directories: vec![],
            files: HashMap::new(),
        }
    }

    pub fn add_dir(&mut self, dir: String) -> &Self {
        self.directories.push(dir);
        self
    }

    pub fn add_file(&mut self, file: String, size: usize) -> &Self {
        self.files.insert(file, size);
        self
    }
}
