use std::borrow::Borrow;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

#[derive(Debug)]
enum VNode {
    File(Rc<VFile>),
    Directory(Rc<VDirectory>)
}

#[derive(Debug)]
struct VFile {
    name: String,
    size: usize
}

#[derive(Debug)]
struct VDirectory {
    items: RefCell<Vec<VNode>>,
    name: String
}

impl VNode {
    fn get_size(&self) -> usize {
        match self {
            VNode::Directory(dir)=> {
                dir.items.borrow().iter().map(|item| item.get_size()).sum()
            },
            VNode::File(file) => file.size
        }
    }
}

fn main() {
    // read testinput line by line
    let input_file = File::open("input").unwrap();
    let mut root = Rc::new(VDirectory {
        items: RefCell::new(Vec::new()),
        name: String::from("")
    });
    let mut path = vec![root.clone()];

    for line in BufReader::new(input_file).lines() {
        let line = line.unwrap();
        if line.starts_with("$") {
            if line.starts_with("$ cd") {
                let name = line[5..].to_string();
                if name == "/" {
                    path = vec![root.clone()];
                    continue;
                }
                if name == ".." {
                    path.pop();
                    continue;
                }
                let directory = VDirectory {
                    items: RefCell::new(Vec::new()),
                    name
                };
                let node = VNode::Directory(Rc::from(directory));
                path.last_mut().unwrap().items.borrow_mut().push(node);
                let directory = match path.last_mut().unwrap().items.borrow_mut().last_mut().unwrap() {
                    VNode::File(_) => panic!("This should never happen"),
                    VNode::Directory(dir) => dir.clone()
                };
                path.push(directory);
            }
        } else {
            let size_string = line[0..line.find(" ").unwrap()].to_string();
            if size_string == "dir" {
                continue;
            }
            let size = size_string.parse::<usize>().unwrap();
            let name = line[line.find(" ").unwrap() + 1..].to_string();
            let file = VFile {
                name,
                size
            };
            let node = VNode::File(Rc::from(file));
            path.last_mut().unwrap().items.borrow_mut().push(node);
        }
    }

    dbg!(&root);
    let root_node = VNode::Directory(root);
    dbg!(root_node.get_size());

    dbg!(get_size_if_smaller(&root_node));
}

fn get_size_if_smaller(node: &VNode) -> usize {
    let node_size = node.get_size();
    match node {
        VNode::Directory(dir) => {
            let others = dir.items.borrow().iter().map(|item| get_size_if_smaller(item)).sum();
            println!("{} {} {} ", dir.name, node_size, others);
            if node_size < 100000 {
                node_size + others
            } else {
                others
            }
        },
        VNode::File(file) => 0
    }
}
