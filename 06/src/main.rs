use std::env;
use std::fs;

use std::fmt;
use std::cmp;


fn path_from_args() -> String {
    return env::args().nth(1).expect("Requires a file path as 1st argument.");
}


fn read_input(input_path: &str) -> String {
    println!("Read inputs from '{input_path}'");
    fs::read_to_string(input_path)
        .expect("Could not read file content.")
}


// TODO: sure they exist in std
fn more(a: usize, b: usize) -> bool {
    return a > b;
}

fn less(a: usize, b: usize) -> bool {
    return a < b;
}

// Need to go through a function to be able to implicitly coerce the type of the tuple
// Otherwise the arms do not have matching types...
fn get_functions(dir: &Facing) -> (fn(usize, usize) -> bool, fn(usize) -> usize) {
    match dir {
        Facing::N | Facing::W => (less, |x| {x+1}),
        Facing::S | Facing::E => (more, |x| {x-1}),
    }
}

fn next_obstacle<T: Iterator>(it: T, pos: usize, dir: &Facing) -> Option<usize>
where
    T::Item: std::ops::Deref<Target = usize>
{
    let (cmp, offset) = get_functions(dir);
    for obstacle in it {
        if cmp(*obstacle, pos) {
            return Option::Some(offset(*obstacle));
        }
    }
    None
}


// Sorted position (index in the zero-index line) of obstacles
type ObstacleLine = Vec<usize>;


struct LabMap {
    //  The horizontal lines of obstacle (top to bottom)
    rows : Vec<ObstacleLine>,
    //  The vertical lines of obstacle (left to right)
    columns : Vec<ObstacleLine>,
}


#[derive(Debug)]
enum Facing {
    N,
    E,
    S,
    W,
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}


impl LabMap {
    fn from_input(content: &str) -> (LabMap, Guard) {
        // Initialize columns (as many as map width)
        let map_width =
            content.lines().nth(0).expect("Gotta have at least 1 line.").
                chars().count();
        let mut result = LabMap {
            rows: Vec::new(),
            columns: Vec::new()
        };
        result.columns.resize(map_width, Vec::new());

        let mut guard = Option::None;

        // Fill in the rows and columns
        for (row_idx, line) in content.lines().enumerate() {
            let mut row = ObstacleLine::new();
            for (column_idx, char) in line.chars().enumerate() {
                if char == '#' {
                    row.push(column_idx);
                    result.columns[column_idx].push(row_idx);
                }
                else if char == '^' {
                    guard = Option::Some(Guard {
                        position: Position {x: column_idx, y: row_idx},
                        direction: Facing::N,
                    });
                }
            }
            result.rows.push(row);
        }
        (result, guard.expect("Guard has to be present on the input."))
    }

    fn collide(&self, pos: Position) -> bool {
        self.rows[pos.y].contains(&pos.x)
        || self.columns[pos.x].contains(&pos.y)
    }
}


impl fmt::Display for LabMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Map (w: {}, h: {})\nrows: ", self.columns.len(), self.rows.len())?;
        for row in &self.rows {
            write!(f, "{row:?}")?;
        }
        write!(f, "\ncolumns: ")?;
        for col in &self.columns {
            write!(f, "{col:?}")?;
        }
        // TODO: there is likely a clever way to return the last result
        write!(f, "")
    }
}


#[derive(Debug)]
enum MoveResult {
    Inside(usize),
    Leaving(usize),
}

#[derive(Debug)]
struct Guard {
    position: Position,
    direction: Facing,
}



impl Guard {
    fn next_candidate(&self) -> Position {
        let x = self.position.x;
        let y = self.position.y;
        match self.direction {
            Facing::N => Position{x, y: y - 1},
            Facing::S => Position{x, y: y + 1},
            Facing::W => Position{x: x - 1, y},
            Facing::E => Position{x: x + 1, y},
        }
    }

    fn rotate(&mut self) {
        match self.direction {
            Facing::N => self.direction = Facing::E,
            Facing::E => self.direction = Facing::S,
            Facing::S => self.direction = Facing::W,
            Facing::W => self.direction = Facing::N,
        }
    }

    fn is_border(&mut self, lab: &LabMap) -> bool {
        let x = self.position.x;
        let y = self.position.y;
        x == 0 || x == (lab.columns.len() - 1)
        || y == 0 || y == (lab.rows.len() - 1)
    }

    fn displace(&mut self, lab: &LabMap, res: &mut ResMap) -> MoveResult {
        let pos: usize;
        let along;
        let line;
        let iter: Box<dyn Iterator<Item = &usize>>;

        match self.direction {
            Facing::N | Facing::S => {
                pos = self.position.x;
                along = & mut self.position.y;
                line = &lab.columns[pos];
            },
            Facing::E | Facing::W => {
                pos = self.position.y;
                along = & mut self.position.x;
                line = &lab.rows[pos];
            },
        };

        match self.direction {
            Facing::N | Facing::W => {
                iter = Box::new(line.iter().rev());
            }
            Facing::S | Facing::E => {
                iter = Box::new(line.iter());
            }
        };

        let new_pos = next_obstacle(iter, *along, &self.direction);

        if let Option::Some(next) = new_pos {
            let displacement = cmp::max(*along, next) - cmp::min(*along, next);
            for i in 0..displacement {
                self.position = self.next_candidate();
                res.toggle(&self.position);
            }
            self.rotate();
            return MoveResult::Inside(displacement);
        }
        else {
            let mut count = 0;
            // TODO: There is an error here, if the guard was leaving at a corner
            // we would miss the last line (because this line would be along a border)
            while !self.is_border(lab) {
                count = count + 1;
                self.position = self.next_candidate();
                res.toggle(&self.position);
            }
            return MoveResult::Leaving(count);
        }
    }
}


struct ResMap {
    positions: Vec<u8>,
    distinct_visited: usize,
    stride: usize,
}


impl ResMap {
    fn make(lab: &LabMap) -> ResMap {
        let mut res = ResMap {
            positions: Vec::new(),
            distinct_visited: 0,
            stride: lab.columns.len(),
        };
        res.positions.resize(res.stride * lab.rows.len(), 0);
        res
    }

    fn toggle(&mut self, pos: &Position) {
        let a = &mut self.positions[self.stride * pos.y + pos.x];
        if *a == 0 {
            self.distinct_visited += 1;
            *a = 1;
        }
    }
}


impl fmt::Display for ResMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let h = self.positions.len() / self.stride;
        write!(f, "ResMap (w: {}, h: {})\n", self.stride, h)?;
        for row in 0..h {
            for c in 0..self.stride {
                if self.positions[row * self.stride + c] == 0 {
                    write!(f, ".")?;
                }
                else {
                    write!(f, "X")?;
                }
            }
            write!(f, "\n")?;
        }
        // TODO: there is likely a clever way to return the last result
        write!(f, "")
    }
}



fn main() {
    let content = read_input(&path_from_args());

    let (lab_map, mut guard) = LabMap::from_input(&content);

    println!("Guard init: {guard:?}\n{lab_map}");

    let mut res_map = ResMap::make(&lab_map);
    res_map.toggle(&guard.position);
    while let MoveResult::Inside(_) = guard.displace(&lab_map, &mut res_map) {
    }
    println!("Resmap:\n{res_map}");

    println!("Visited {}.", res_map.distinct_visited);

}
