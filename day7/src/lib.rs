pub fn run(input: &String, _part: i32) -> i32 {
    let mut root: Directory = Directory::new(&String::from("/"));
    let curr_dir: &mut Directory = &mut root;
    let mut dir_stack: Vec<&Directory> = Vec::new();

    curr_dir.add_dir(&String::from("a"));
    curr_dir.add_file(&String::from("Hey"), 142);
    change_dir(curr_dir, &mut dir_stack, &String::from("a"));
    curr_dir.add_file(&String::from("Hay"), 32112);
    return 0;
}

fn change_dir(curr_dir: &mut Directory, dir_stack: &mut Vec<&Directory>, new_dirr: &String) {
    if new_dirr == ".." {

    } else {
        dir_stack.push(curr_dir);
        let curr_dir = match curr_dir.get_dir(new_dirr) {
            Some(Directory) => Directory,
            None => panic!("Didn't get find {} in {}", new_dirr, curr_dir.name),
        };
    }
    
}

struct Directory {
    name: String,
    directories: Vec<Directory>,
    files: Vec<File>,
}

impl Directory {
    fn new(name: &String) -> Directory {
        let name = name.clone();
        let directories: Vec<Directory> = Vec::new();
        let files: Vec<File> = Vec::new();
        Directory { name, directories, files }
    }

    fn add_dir(&mut self, name: &String) {
        self.directories.push(Directory::new(name));
    }

    fn add_file(&mut self, name: &String, size: i32) {
        self.files.push(File::new(name, size));
    }

    fn get_dir(&mut self, name: &String) -> Option<&Directory> {
        for dir in &self.directories {
            if dir.name.eq(name) {
                return Some(&dir)
            }
        }
        None
    }
}

struct File {
    name: String,
    size: i32,
}

impl File {
    fn new(name: &String, size: i32) -> File {
        let name = name.clone();
        File { name, size }
    }
}