use std::{fs::File, io::{BufReader, BufRead}, fmt::{Display, Formatter}};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Item {
    Empty,
    Solid,
    Movable
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Item::Empty => write!(f, "."),
            Item::Solid => write!(f, "#"),
            Item::Movable => write!(f, "O")
        }
    }
}

fn printmap(map: &Vec<Vec<Item>>) {
    for row in map {
        for item in row {
            print!("{}", item);
        }
        println!();
    }
}

fn value(map: &Vec<Vec<Item>>) -> usize {
    let mut value = 0;
    for (i, row) in map.iter().enumerate() {
        for item in row {
            match item {
                Item::Movable => value += map.len() - i,
                _ => ()
            }
        }
    }
    value
}

fn north(map: &mut Vec<Vec<Item>>) {
    for x in 0..map.len() {
        let mut rockcount = vec![];
        let mut _rock = 0;
        let mut first_y = 0;

        for y in 0..map.len() {
            match map[y][x] {
                Item::Movable => _rock += 1,
                Item::Solid => {
                    rockcount.push((first_y, _rock));
                    _rock = 0;
                    first_y = y + 1;
                },
                _ => ()
            }
        }
        if _rock > 0 {
            rockcount.push((first_y, _rock));
        }

        let mut rockshift = 0;

        for y in 0..map.len() {
            if map[y][x] == Item::Solid {
                rockshift += 1;
                continue;
            }
            map[y][x] = if rockshift < rockcount.len() && rockcount[rockshift].0 + rockcount[rockshift].1 > y {
                Item::Movable
            } else {
                Item::Empty
            }
        }
    }
}

fn south(map: &mut Vec<Vec<Item>>) {
    for x in 0..map.len() {
        let mut rockcount = vec![];
        let mut _rock = 0;
        let mut first_y = 0;

        for y in (0..map.len()).rev() {
            match map[y][x] {
                Item::Movable => _rock += 1,
                Item::Solid => {
                    rockcount.push((first_y, _rock));
                    _rock = 0;
                    first_y = y + 1;
                },
                _ => ()
            }
        }
        if _rock > 0 {
            rockcount.push((first_y, _rock));
        }

        let mut rockshift = 0;

        for y in 0..map.len() {
            if map[y][x] == Item::Solid {
                rockshift += 1;
                continue;
            }
            map[y][x] = if rockshift < rockcount.len() && rockcount[rockshift].0 + rockcount[rockshift].1 < y {
                Item::Movable
            } else {
                Item::Empty
            }
        }
    }
}

fn east(map: &mut Vec<Vec<Item>>) {
    for y in 0..map.len() {
        let mut rockcount = vec![];
        let mut _rock = 0;
        let mut first_x = 0;

        for x in (0..map.len()).rev() {
            match map[y][x] {
                Item::Movable => _rock += 1,
                Item::Solid => {
                    rockcount.push((first_x, _rock));
                    _rock = 0;
                    first_x = x + 1;
                },
                _ => ()
            }
        }
        if _rock > 0 {
            rockcount.push((first_x, _rock));
        }

        let mut rockshift = 0;

        for x in 0..map.len() {
            if map[y][x] == Item::Solid {
                rockshift += 1;
                continue;
            }
            map[y][x] = if rockshift < rockcount.len() && rockcount[rockshift].0 + rockcount[rockshift].1 > x {
                Item::Movable
            } else {
                Item::Empty
            }
        }
    }
}

fn west(map: &mut Vec<Vec<Item>>) {
    for y in 0..map.len() {
        let mut rockcount = vec![];
        let mut _rock = 0;
        let mut first_x = 0;

        for x in 0..map.len() {
            match map[y][x] {
                Item::Movable => _rock += 1,
                Item::Solid => {
                    rockcount.push((first_x, _rock));
                    _rock = 0;
                    first_x = x + 1;
                },
                _ => ()
            }
        }
        if _rock > 0 {
            rockcount.push((first_x, _rock));
        }

        let mut rockshift = 0;

        for x in 0..map.len() {
            if map[y][x] == Item::Solid {
                rockshift += 1;
                continue;
            }
            map[y][x] = if rockshift < rockcount.len() && rockcount[rockshift].0 + rockcount[rockshift].1 < x {
                Item::Movable
            } else {
                Item::Empty
            }
        }
    }
}

fn main() {
    let mut map = vec![];

    let file = File::open("input1").expect("File not found");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut row = vec![];
        for c in line.chars() {
            match c {
                '.' => row.push(Item::Empty),
                '#' => row.push(Item::Solid),
                'O' => row.push(Item::Movable),
                _ => panic!("Unknown item")
            }
        }
        map.push(row);
    }

    printmap(&map);

    println!("\nnorth");
    north(&mut map);
    printmap(&map);
    println!("\nwest");
    west(&mut map);
    printmap(&map);
    println!("\nsouth");
    south(&mut map);
    printmap(&map);
    println!("\neast");
    east(&mut map);

    printmap(&map);

    println!("Value: {}", value(&map));
}
