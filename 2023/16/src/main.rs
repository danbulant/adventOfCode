use std::{fs::File, io::{BufReader, BufRead, Write}, ops::Add, fmt::Debug, hash::Hasher};

fn main() {
    part1();
    part2();
    // let mut values = HashSet::new();
    // for x in 0..110 {
    //     for y in 0..110 {
    //         let vecs = [
    //             VEC_RIGHT,
    //             VEC_LEFT,
    //             VEC_UP,
    //             VEC_DOWN
    //         ];
    //         for vec in vecs {
    //             // let vecval = ((vec.x+2)*vec.x.abs()+(vec.y+3)*vec.y.abs() - 1) as usize;
    //             let vecval = (vec.x+1)*vec.x.abs()+(vec.y+2)*vec.y.abs();
    //             let val = (x * 111 + y + 1) << 2 | vecval as usize;
    //             println!("{} {} {:?} val={} vecval(plex)={}", x, y, vec, val, vecval);
    //             if !values.insert(val) {
    //                 println!("Collision at {},{} with {:?} {}", x, y, vec, val);
    //                 exit(1);
    //             }
    //         }
    //     }
    // }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Type {
    /// Mirror bottom-to-top (/)
    MirrorBT,
    /// Mirror top-to-bottom (\)
    MirrorTB,
    /// Split horizontally (-)
    SplitHorizontal,
    /// Split vertically (|)
    SplitVertical,
}

fn vectors_from_split(split: Type) -> (Vector, Vector) {
    match split {
        Type::SplitHorizontal => (Vector { x: -1, y: 0}, Vector { x: 1, y: 0 }),
        Type::SplitVertical => (Vector { x: 0, y: -1}, Vector { x: 0, y: 1 }),
        _ => panic!("Not a split")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MapPoint {
    point: Point,
    mirror: Type
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector {
    x: isize,
    y: isize
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Beam {
    point: Point,
    vector: Vector
}

// const VECTOR_VALUES_HASH: [usize; 13] = [
//     0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3
// ];

impl Beam {
    fn to_nums(&self) -> (usize, usize) {
        let vecval = (self.vector.x+1)*self.vector.x.abs()+(self.vector.y+2)*self.vector.y.abs();
        let val = (self.point.x * 110 + self.point.y + 1) << 2 | vecval as usize;
        (val >> 6, 1 << (val & 63))
        // 0b1000000000000000000000000000000000000000000000000000000000000000
    }
}

#[derive(Debug, Default)]
struct BeamHasher {
    nums: u64
}

impl Hasher for BeamHasher {
    fn write(&mut self, bytes: &[u8]) {
        // assign bytes directly to nums - this hasher will always get 4 u8s, just once
        // self.nums = 0;
        for i in 0..4 {
            self.nums ^= (bytes[i] as u64) << (i * 8);
        }
    }

    fn finish(&self) -> u64 {
        self.nums
    }
}

impl std::hash::Hash for Beam {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&[
            self.point.x as u8,
            self.point.y as u8,
            self.vector.x as u8,
            self.vector.y as u8
        ]);
    }
}

impl Vector {
    fn map_by_mirror(&self, mirror: Type) -> Vector {
        match mirror {
            Type::MirrorBT => match self {
                &VEC_RIGHT => VEC_UP,
                &VEC_LEFT => VEC_DOWN,
                &VEC_UP => VEC_RIGHT,
                &VEC_DOWN => VEC_LEFT,
                _ => panic!("Invalid vec")
            },
            Type::MirrorTB => match self {
                &VEC_RIGHT => VEC_DOWN,
                &VEC_LEFT => VEC_UP,
                &VEC_UP => VEC_LEFT,
                &VEC_DOWN => VEC_RIGHT,
                _ => panic!("Invalid vec")
            },
            _ => panic!("Not a mirror")
        }
    }

    fn map_to_split(&self) -> Type {
        match self {
            &VEC_RIGHT => Type::SplitHorizontal,
            &VEC_LEFT => Type::SplitHorizontal,
            &VEC_UP => Type::SplitVertical,
            &VEC_DOWN => Type::SplitVertical,
            _ => panic!("Invalid vec")
        }
    }

    fn will_go_oob(&self, point: Point, max_point: Point) -> bool {
        let nx = point.x as isize + self.x;
        let ny = point.y as isize + self.y;

        nx < 0 || ny < 0 || nx >= max_point.x as isize || ny >= max_point.y as isize
    }

    fn is_vertical(&self) -> bool {
        self.x == 0
    }

    fn is_horizontal(&self) -> bool {
        self.y == 0
    }

    fn is_positive(&self) -> bool {
        self.x > 0 || self.y > 0
    }

    fn is_negative(&self) -> bool {
        self.x < 0 || self.y < 0
    }
}
impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            x: (self.x as isize + rhs.x) as usize,
            y: (self.y as isize + rhs.y) as usize
        }
    }
}

const VEC_RIGHT : Vector = Vector { x: 1, y: 0 };
const VEC_LEFT : Vector = Vector { x: -1, y: 0 };
const VEC_UP : Vector = Vector { x: 0, y: -1 };
const VEC_DOWN : Vector = Vector { x: 0, y: 1 };

fn print_energized(energized: &Vec<Vec<bool>>) -> Result<(), std::io::Error> {
    let mut lock = std::io::stdout().lock();
    for y in 0..energized.len() {
        for x in 0..energized[y].len() {
            write!(lock, "{}", if energized[y][x] { '#' } else { '.' })?;
        }
        writeln!(lock)?;
    }
    Ok(())
}

// fn append_if_not_in<T: PartialEq + Debug>(vec: &mut Vec<T>, item: T) {
//     if !vec.contains(&item) {
//         vec.push(item);
//     }
// }
fn set_range<T: Copy>(vec: &mut Vec<T>, range: std::ops::Range<usize>, item: T) {
    for i in range {
        vec[i] = item;
    }
}
fn set_range_in<T: Copy>(vec: &mut Vec<Vec<T>>, range: std::ops::Range<usize>, item: T, offset: usize) {
    for i in range {
        vec[i][offset] = item;
    }
}

fn mark(vec: Vector, point: Point, otherpos: usize, energized: &mut Vec<Vec<bool>>) {
    if vec.is_vertical() {
        if vec.is_positive() {
            set_range_in(energized, point.y..otherpos, true, point.x);
        } else {
            set_range_in(energized, otherpos..(point.y + 1), true, point.x);
        }
    } else {
        if vec.is_positive() {
            set_range(&mut energized[point.y], point.x..otherpos, true);
        } else {
            set_range(&mut energized[point.y], otherpos..(point.x + 1), true);
        }
    }
}

fn get_map() -> Vec<Vec<char>> {
    let file = File::open("input2").expect("File not found");
    let reader = BufReader::new(file);
    let map : Vec<Vec<char>> = reader.lines().map(|l| l.unwrap().chars().collect()).collect();
    map
}

fn get_rows_and_cols(map: Vec<Vec<char>>) -> (Vec<Vec<MapPoint>>, Vec<Vec<MapPoint>>) {
    let rows: Vec<Vec<MapPoint>> = map.iter().enumerate().map(|(y, row)| {
        row
            .iter()
            .enumerate()
            .filter(|(_, c)| **c != '.')
            .map(|(x, c)| {
                MapPoint {
                    point: Point { x, y },
                    mirror: match c {
                        '/' => Type::MirrorBT,
                        '\\' => Type::MirrorTB,
                        '-' => Type::SplitHorizontal,
                        '|' => Type::SplitVertical,
                        _ => panic!("Invalid char")
                    }
                }
            }).collect()
    }).collect();
    let columns: Vec<Vec<MapPoint>> = (0..map[0].len()).map(|x| {
        (0..map.len())
            .filter(|y| map[*y][x] != '.')
            .map(|y| {
                MapPoint {
                    point: Point { x, y },
                    mirror: match map[y][x] {
                        '/' => Type::MirrorBT,
                        '\\' => Type::MirrorTB,
                        '-' => Type::SplitHorizontal,
                        '|' => Type::SplitVertical,
                        _ => panic!("Invalid char")
                    }
                }
            }).collect()
    }).collect();

    (rows, columns)
}

fn setup() -> (Point, Vec<Vec<MapPoint>>, Vec<Vec<MapPoint>>) {
    let map = get_map();
    // let energized : Vec<Vec<bool>> = map.iter().map(|l| l.iter().map(|_| false).collect()).collect();
    let max_point = Point { x: map[0].len(), y: map.len() };
    let (rows, columns) = get_rows_and_cols(map);

    (max_point, rows, columns)
}

fn get_count(max_point: Point, rows: Vec<Vec<MapPoint>>, columns: Vec<Vec<MapPoint>>, beam: Beam) -> usize {
    let mut energized : Vec<Vec<bool>> = vec![vec![false; max_point.x]; max_point.y];
    // let mut beams_set = HashSet::<Beam, BuildHasherDefault<BeamHasher>>::default();
    let mut beams_set_vec = vec![0; (max_point.x + 1) * (max_point.y + 1) / 8 + 1];
    let mut beams : Vec<Beam> = vec![beam];
    // beams_set.insert(beams[0]);
    let b0 = beams[0].to_nums();
    beams_set_vec[b0.0] = b0.1;

    let mut offset = 0;

    while beams.len() > offset {
        let beam = &beams[offset];
        offset += 1;

        let vec = beam.vector;
        let vec_is_vertical = vec.is_vertical();
        let mut relevant = {
            (if vec_is_vertical {
                columns[beam.point.x].iter()
            } else {
                rows[beam.point.y].iter()
            })
            .filter(|mp| {
                let mpp = if vec_is_vertical { mp.point.y } else { mp.point.x };
                let beamp = if vec_is_vertical { beam.point.y } else { beam.point.x };
                // ignore splits that are the same direction as the beam
                mp.mirror != vec.map_to_split() &&
                (mpp >= beamp && vec.is_positive() || mpp <= beamp && vec.is_negative())
            })
        };
        let closest = if vec.is_positive() {
            relevant.next()
        } else {
            relevant.last()
        };
        match closest {
            None => {
                mark(vec, beam.point, if vec.is_positive() { max_point.x } else { 0 }, &mut energized);
            },
            Some(mappoint) => {
                mark(vec, beam.point, (if vec_is_vertical { mappoint.point.y } else { mappoint.point.x } ) + (if vec.is_positive() { 1 } else { 0 }), &mut energized);
                let mut vecs = vec![];
                if mappoint.mirror == Type::SplitHorizontal || mappoint.mirror == Type::SplitVertical {
                    let (v1, v2) = vectors_from_split(mappoint.mirror);
                    vecs.push(v1);
                    vecs.push(v2);
                } else {
                    vecs.push(vec.map_by_mirror(mappoint.mirror));
                }
                let point = mappoint.point;
                for v in vecs {
                    if !v.will_go_oob(point, max_point) {
                        let new_point = point + v;
                        let beam = Beam {
                            point: new_point,
                            vector: v
                        };
                        // append_if_not_in(&mut beams, beam);
                        // if beams_set.insert(beam) {
                        let bnum = beam.to_nums();
                        if beams_set_vec[bnum.0] & bnum.1 == 0 {
                            beams_set_vec[bnum.0] |= bnum.1;
                            beams.push(beam);
                        }
                    }
                }
            }
        }
    }
    // print_energized(&energized).unwrap();

    let count = energized.iter().map(|row| row.iter().filter(|&&b| b).count()).sum::<usize>();
    count
}

fn part1() {
    let (max_point, rows, columns) = setup();
    let count = get_count(max_point, rows, columns, Beam {
        point: Point { x: 0, y: 0 },
        vector: VEC_RIGHT
    });
    println!("Part 1 count: {}", count);
}

fn part2() {
    let (max_point, rows, columns) = setup();
    let mut count = 0;
    for x in 0..max_point.x {
        for y in 0..max_point.y {
            if x != 0 && y != 0 && x != max_point.x - 1 && y != max_point.y - 1 {
                continue;
            }
            let mut vecs = vec![];
            if x > 0 {
                vecs.push(VEC_LEFT);
            }
            if x == 0 {
                vecs.push(VEC_RIGHT);
            }
            if y > 0 {
                vecs.push(VEC_UP);
            }
            if y == 0 {
                vecs.push(VEC_DOWN);
            }
            for vector in vecs {
                let count_here = get_count(max_point, rows.clone(), columns.clone(), Beam {
                    point: Point { x, y },
                    vector
                });
                if count_here > count {
                    count = count_here;
                    println!("New max: {} at {},{}", count, x, y);
                }
            }
        }
    }
    println!("Max count: {}", count);
}