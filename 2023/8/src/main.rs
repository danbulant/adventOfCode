use std::{io::{BufReader, BufRead}, fs::File, ops::Index};
use std::collections::HashMap;
use std::sync::mpsc;

struct Node {
    left: String,
    right: String
}
enum Instruction {
    Left,
    Right
}

fn main() {
    let values = [[18156, 36313], [11652, 23305], [21408, 42817], [12736, 25473], [14362, 28725], [15988, 31977]];
    let lowest_value = values.iter().map(|v| v[1]).min().unwrap();
    let mut num: i64 = values[1][0];
    let mut printcount = 0;
    dbg!(lowest_value);

    'main: loop {
        num += lowest_value;
        for value in &values {
            if num % value[1] != value[0] {
                printcount += 1;
                if printcount % 100000000 == 0 {
                    dbg!(num);
                }
                // if value[0] != 18156 {
                //     dbg!(num, value[1], value[0], num % value[1]);
                // }
                continue 'main;
            }
        }
        break;
    }

    dbg!(num);

    // let file = BufReader::new(File::open("./input2").unwrap());
    // let mut instructions = vec![];
    // let mut nodes = HashMap::new();

    // for line in file.lines() {
    //     let line = line.unwrap();
    //     if instructions.is_empty() {
    //         let chars = line.chars().collect::<Vec<char>>();
    //         // L = Left, R = Right
    //         for c in chars {
    //             match c {
    //                 'L' => instructions.push(Instruction::Left),
    //                 'R' => instructions.push(Instruction::Right),
    //                 _ => ()
    //             }
    //         }
    //         continue;
    //     }
    //     if line.is_empty() {
    //         continue;
    //     }
    //     let name = line.split(" ").next().unwrap();
    //     let left = line.split("(").nth(1).unwrap().split(",").next().unwrap();
    //     let right = line.split("(").nth(1).unwrap().split(",").nth(1).unwrap().strip_suffix(")").unwrap().trim();

    //     nodes.insert(name.to_string(), Node { left: left.to_string(), right: right.to_string() });
    // }

    // let mut instruction_pointer = 0;
    // let mut current_nodes = nodes.keys().filter(|k| k.ends_with("A")).collect::<Vec<&String>>();
    // /// Array of arrays corresponding to current_nodes, with each value in the array being the step in which the node reaches Z node
    // let mut z_values = Arc::new(RwLock::new(vec![Mutex::new(vec![]); current_nodes.len()]));

    // dbg!(&current_nodes);

    // let mut count = 0;

    // let z_values2 = z_values.clone();
    // let main_join_thread = thread::spawn(move || {
    //     // If there's a common number between z_values, stop and return
    //     loop {
    //         let z_values = z_values2.read().unwrap();
    //         let mut common_values = z_values.get(0).unwrap().lock().unwrap().clone();
    //         for i in 1..z_values.len() {
    //             let values = z_values.get(i).unwrap().lock().unwrap().clone();
    //             common_values = common_values.iter().filter(|v| values.contains(v)).collect::<Vec<&usize>>();
    //         }
    //         if !common_values.is_empty() {
    //             return common_values[0].clone();
    //         }
    //     }
    // });

    // let mut offset = 0;
    // for node in current_nodes {
    //     thread::spawn(move || {
    //         let mut node = node;
    //         let offset = offset;
    //     });
    //     offset += 1;
    // }

    // let val = main_join_thread.join().unwrap();
    // dbg!(val);

    // while !current_nodes.iter().all(|n| n.ends_with("Z")) {
    //     let mut next_nodes = vec![];
    //     for node in current_nodes {
    //         let node = nodes.get(node).unwrap();
    //         match instructions[instruction_pointer] {
    //             Instruction::Left => next_nodes.push(&node.left),
    //             Instruction::Right => next_nodes.push(&node.right)
    //         }
    //     }
    //     instruction_pointer += 1;
    //     instruction_pointer %= instructions.len();
    //     count += 1;
    //     current_nodes = next_nodes;
    // }

    // dbg!(count);
    // println!("{count}");
}
