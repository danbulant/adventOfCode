use std::fmt::{Formatter, Display};


#[derive(Debug, PartialEq, Clone)]
enum Place {
    Empty,
    East,
    South
}

impl Display for Place {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Place::Empty => write!(f, "."),
            Place::East => write!(f, ">"),
            Place::South => write!(f, "v")
        }
    }
}

impl Place {
    fn from_char(c: char) -> Place {
        match c {
            '.' => Place::Empty,
            '>' => Place::East,
            'v' => Place::South,
            _ => panic!("Unknown char: {}", c)
        }
    }
}

struct Board(Vec<Vec<Place>>);

impl Board {
    fn push(&mut self, row: Vec<Place>) {
        self.0.push(row);
    }

    fn tick(&mut self) -> bool {
        let mut moved = false;
        let mut new_board = self.0.clone();
        for (y, row) in self.0.iter().enumerate() {
            for (x, place) in row.iter().enumerate() {
                match place {
                    Place::Empty => (),
                    Place::East => {
                        if x + 1 < row.len() && self.0[y][x + 1] == Place::Empty {
                            new_board[y][x + 1] = Place::East;
                            new_board[y][x] = Place::Empty;
                            moved = true;
                        } else if x + 1 == row.len() && self.0[y][0] == Place::Empty {
                            new_board[y][0] = Place::East;
                            new_board[y][x] = Place::Empty;
                            moved = true;
                        }
                    },
                    Place::South => ()
                }
            }
        }
        self.0 = new_board;
        new_board = self.0.clone();
        for (y, row) in self.0.iter().enumerate() {
            for (x, place) in row.iter().enumerate() {
                match place {
                    Place::Empty => (),
                    Place::East => (),
                    Place::South => {
                        if y + 1 < self.0.len() && self.0[y + 1][x] == Place::Empty {
                            new_board[y + 1][x] = Place::South;
                            new_board[y][x] = Place::Empty;
                            moved = true;
                        } else if y + 1 == self.0.len() && self.0[0][x] == Place::Empty {
                            new_board[0][x] = Place::South;
                            new_board[y][x] = Place::Empty;
                            moved = true;
                        }
                    }
                }
            }
        }
        self.0 = new_board;
        moved
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for row in &self.0 {
            for place in row {
                write!(f, "{}", place)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut places = Board(Vec::new());
    let lines = input.lines();
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Place::from_char(c));
        }
        places.push(row);
    }
    println!("{}", places);
    println!("");
    let mut sum = 0;
    while places.tick() {
        println!("{}", places);
        println!("");
        sum += 1;
        println!("{sum}\n");
    }
    println!("\n{}", sum + 1);
}