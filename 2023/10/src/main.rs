use std::{io::{BufReader, BufRead}, fs::File, fmt::{Formatter, Display}};

#[derive(Debug, Clone, PartialEq)]
enum Directions {
    North,
    South,
    East,
    West
}

#[derive(Debug, PartialEq)]
enum Pipes {
    Vertical = '|' as isize,
    Horizontal = '-' as isize,
    NE = 'L' as isize,
    NW = 'J' as isize,
    SW = '7' as isize,
    SE = 'F' as isize,
    Start = 'S' as isize,
    None = '.' as isize
}

impl Display for Pipes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vertical => write!(f, "│"),
            Self::Horizontal => write!(f, "─"),
            Self::NE => write!(f, "└"),
            Self::NW => write!(f, "┘"),
            Self::SW => write!(f, "┐"),
            Self::SE => write!(f, "┌"),
            Self::Start => write!(f, "S"),
            Self::None => write!(f, ".")
        }
    }
}

impl Pipes {
    fn new(c: char) -> Self {
        match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            'S' => Self::Start,
            _ => Self::None
        }
    }

    fn get_change_from(&self, prev: &PosChange) -> PosChange {
        match self {
            Self::Vertical => {
                if prev.y == -1 {
                    PosChange::new(0, 1)
                } else {
                    PosChange::new(0, -1)
                }
            },
            Self::Horizontal => {
                if prev.x == -1 {
                    PosChange::new(1, 0)
                } else {
                    PosChange::new(-1, 0)
                }
            },
            Self::NE => {
                if prev.x == 0 {
                    PosChange::new(1, 0)
                } else {
                    PosChange::new(0, -1)
                }
            },
            Self::NW => {
                if prev.x == 0 {
                    PosChange::new(-1, 0)
                } else {
                    PosChange::new(0, -1)
                }
            },
            Self::SW => {
                if prev.x == 0 {
                    PosChange::new(-1, 0)
                } else {
                    PosChange::new(0, 1)
                }
            },
            Self::SE => {
                if prev.x == 0 {
                    PosChange::new(1, 0)
                } else {
                    PosChange::new(0, 1)
                }
            },
            _ => PosChange::new(0, 0)
        }
    }

    fn get_directions(&self) -> Vec<Directions> {
        match self {
            Self::Vertical => vec![Directions::North, Directions::South],
            Self::Horizontal => vec![Directions::East, Directions::West],
            Self::NE => vec![Directions::North, Directions::East],
            Self::NW => vec![Directions::North, Directions::West],
            Self::SW => vec![Directions::South, Directions::West],
            Self::SE => vec![Directions::South, Directions::East],
            _ => vec![]
        }
    }

    fn connects_from(&self, prev: &PosChange) -> bool {
        match self {
            Self::Vertical => prev.x == 0,
            Self::Horizontal => prev.y == 0,
            Self::NE => prev.x == 1 || prev.y == -1,
            Self::NW => prev.x == -1 || prev.y == -1,
            Self::SW => prev.x == -1 || prev.y == 1,
            Self::SE => prev.x == 1 || prev.y == 1,
            _ => false
        }
    }

    fn detect_type(directions: &[Directions; 2]) -> Self {
        match directions {
            [Directions::North, Directions::South] => Self::Vertical,
            [Directions::East, Directions::West] => Self::Horizontal,
            [Directions::North, Directions::East] => Self::NE,
            [Directions::North, Directions::West] => Self::NW,
            [Directions::South, Directions::West] => Self::SW,
            [Directions::South, Directions::East] => Self::SE,
            _ => Self::None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Pos {
    x: usize,
    y: usize
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct PosChange {
    x: isize,
    y: isize
}

impl PosChange {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn will_underflow(&self, pos: Pos) -> bool {
        (pos.x as isize + self.x) < 0 || (pos.y as isize + self.y) < 0
    }
}

impl From<PosChange> for Directions {
    fn from(pos: PosChange) -> Self {
        match pos {
            PosChange { x: 0, y: -1 } => Self::North,
            PosChange { x: 0, y: 1 } => Self::South,
            PosChange { x: -1, y: 0 } => Self::West,
            PosChange { x: 1, y: 0 } => Self::East,
            _ => panic!("Invalid direction")
        }
    }
}

impl From<Directions> for PosChange {
    fn from(dir: Directions) -> Self {
        match dir {
            Directions::North => Self::new(0, -1),
            Directions::South => Self::new(0, 1),
            Directions::West => Self::new(-1, 0),
            Directions::East => Self::new(1, 0)
        }
    }
}

impl std::ops::Sub for Pos {
    type Output = PosChange;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { x: (self.x as isize - rhs.x as isize), y: (self.y as isize - rhs.y as isize) }
    }
}

impl std::ops::Add<&PosChange> for Pos {
    type Output = Self;

    fn add(self, rhs: &PosChange) -> Self::Output {
        Self { x: (self.x as isize + rhs.x) as usize, y: (self.y as isize + rhs.y) as usize }
    }
}
impl std::ops::Add<PosChange> for Pos {
    type Output = Self;

    fn add(self, rhs: PosChange) -> Self::Output {
        std::ops::Add::<&PosChange>::add(self, &rhs)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum MainLoop {
    None,
    Loop,
    Inside
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Pipes>>,
    dist_map: Vec<Vec<Option<usize>>>,
    pos: Pos,
    main_loop: Vec<Vec<MainLoop>>
}

struct DistMap(Vec<Vec<Option<usize>>>);

impl Display for DistMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            for col in row {
                match col {
                    Some(x) => write!(f, "{:03} ", x)?,
                    None => write!(f, "### ")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.map.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                match self.main_loop[y][x] {
                    MainLoop::None => write!(f, "{}", col)?,
                    MainLoop::Loop => write!(f, "\x1b[31m{}\x1b[0m", col)?,
                    MainLoop::Inside => write!(f, "▉")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Map {
    /// Uses pos as the position of the start pipe, so it must be called before any traversals happen
    fn detect_start_pipe(&mut self) {
        let mut directions = vec![];

        for x in -1isize..=1 {
            for y in -1isize..=1 {
                if x == 0 && y == 0 { continue; }
                if x != 0 && y != 0 { continue; }
                let start_change = PosChange::new(x, y);
                if start_change.will_underflow(self.pos) { continue; }
                let pos = self.pos + start_change;
                let change = self.pos - pos;
                if self.get(pos).connects_from(&change) {
                    directions.push(start_change.into());
                }
            }
        }

        if directions.len() != 2 {
            panic!("Invalid start pipe");
        }

        let second = directions.pop().unwrap();
        let first = directions.pop().unwrap();

        let directions = [first, second];

        dbg!(&directions);

        let pipe = Pipes::detect_type(&directions);

        if pipe == Pipes::None {
            panic!("Invalid start pipe - pipe not found");
        }

        self.map[self.pos.y][self.pos.x] = pipe;
    }

    fn get(&self, pos: Pos) -> &Pipes {
        &self.map[pos.y][pos.x]
    }

    fn set_dist_if_lower(&mut self, pos: Pos, dist: usize) {
        if let Some(old_dist) = self.dist_map[pos.y][pos.x] {
            if old_dist <= dist { return; }
        }
        self.dist_map[pos.y][pos.x] = Some(dist);
    }

    fn calc_dist_recurse(&mut self, pos: Pos, prev: PosChange, dist: usize) {
        if pos == self.pos { return; }
        let pipe = self.get(pos);
        let change = pipe.get_change_from(&prev);

        self.main_loop[pos.y][pos.x] = MainLoop::Loop;

        self.set_dist_if_lower(pos, dist);

        self.calc_dist_recurse(pos + change, pos - (pos + change), dist + 1)
    }
}

fn parse_map() -> Map {
    let file = BufReader::new(File::open("./input2").unwrap());
    let mut map = Map { map: vec![], pos: Pos::new(0, 0), dist_map: vec![], main_loop: vec![] };

    for (y, line) in file.lines().enumerate() {
        let line = line.unwrap();
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                map.pos = Pos::new(x, y);
            }
            row.push(Pipes::new(c));
        }
        map.dist_map.push(vec![None; row.len()]);
        map.main_loop.push(vec![MainLoop::None; row.len()]);
        map.map.push(row);
    }

    map.detect_start_pipe();
    map.set_dist_if_lower(map.pos, 0);
    map.main_loop[map.pos.y][map.pos.x] = MainLoop::Loop;

    let dir: PosChange = map.get(map.pos).get_directions().get(0).unwrap().clone().into();
    let dir2: PosChange = map.get(map.pos).get_directions().get(1).unwrap().clone().into();

    map.calc_dist_recurse(map.pos + dir, map.pos - (map.pos + dir), 1);
    map.calc_dist_recurse(map.pos + dir2, map.pos - (map.pos + dir2), 1);

    map
}

fn part1() {
    let map = parse_map();
    let distmap = DistMap(map.dist_map.clone());
    print!("{distmap}");

    let mut max_dist = 0;

    for row in map.dist_map.iter() {
        for col in row {
            if let Some(dist) = col {
                if *dist > max_dist {
                    max_dist = *dist;
                }
            }
        }
    }

    println!("Max dist: {}", max_dist);
}

fn main() {
    let mut map = parse_map();

    println!("{map}");

    let mut marked_positions = vec![];

    for (y, row) in map.main_loop.iter().enumerate() {
        let mut is_enabled = false;
        let mut in_dir = None;
        for (x, col) in row.iter().enumerate() {
            if *col == MainLoop::Loop {
                let pipe = map.get(Pos::new(x, y));
                match pipe {
                    Pipes::Horizontal => {},
                    Pipes::Vertical => {
                        is_enabled = !is_enabled;
                    },
                    Pipes::NE => {
                        in_dir = Some(Directions::North);
                    },
                    Pipes::NW => {
                        if in_dir != Some(Directions::North) {
                            is_enabled = !is_enabled;
                        }
                    },
                    Pipes::SE => {
                        in_dir = Some(Directions::South);
                    },
                    Pipes::SW => {
                        if in_dir != Some(Directions::South) {
                            is_enabled = !is_enabled;
                        }
                    },
                    _ => {}
                }
            }
            if is_enabled && *col == MainLoop::None {
                marked_positions.push(Pos::new(x, y));
            }
        }
    }

    dbg!(&marked_positions);
    let len = marked_positions.len();

    for pos in marked_positions {
        map.main_loop[pos.y][pos.x] = MainLoop::Inside;
    }

    println!("{map}");

    println!("len: {len}");
}