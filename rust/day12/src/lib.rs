use std::collections::{HashMap, BinaryHeap, HashSet};

type Cost = u64;
type Cell = char;

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Hash)]
pub struct XY {
    pub x: i64, pub y: i64
}

impl Into<XY> for (usize, usize) {
    fn into(self) -> XY {
        XY { x: self.0 as i64, y: self.1 as i64 }
    }
}

impl Into<(usize, usize)> for XY {
    fn into(self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}


struct Elevation {
    moves: Vec<Cell>,
}

impl Elevation {
    fn new() -> Self {
        let mut moves = Vec::new();
        moves.push('S');
        moves.extend('a'..='z');
        moves.push('E');
        Self {
            moves
        }
    }

    fn position(&self, of: Cell) -> Option<usize> {
        self.moves.iter().position(|&cell| cell == of)
    }

    fn is_possible(&self, from: Cell, to: Cell) -> bool {
        match (self.position(from), self.position(to)) {
            (Some(from_i), Some(to_i)) => from_i + 1 >= to_i,
            _                                        => false
        }
    }
}

pub struct Grid {
    cells: Vec<Vec<Cell>>,
    pub height: usize,
    pub width: usize,
    elevation: Elevation,
}

impl Grid {
    pub fn new(map: String) -> Self {
        let cells: Vec<Vec<Cell>> = map
            .trim()
            .split('\n')
            .map(|xs| xs.chars().collect()).collect();

        Self {
            height: cells.len(),
            width: cells[0].len(),
            cells,
            elevation: Elevation::new(),
        }
    }

    pub fn print(&self, highlight: Vec<XY>) {
        let mut path: HashSet<(usize, usize)> = HashSet::new();
        for xy in highlight {
            path.insert(xy.into());
        }

        for (y, xs) in self.cells.iter().enumerate() {
            let mut row = Vec::with_capacity(xs.len());
            for (x, chr) in xs.iter().enumerate() {
                if path.contains(&(x, y)) {
                    row.push(format!("\x1b[103m{}\x1b[0m", chr));
                } else {
                    row.push(format!("\x1b[47m{}\x1b[0m", chr));
                }
            }
            println!("{}", row.join(""));
        }
    }

    pub fn find(&self, target: Cell) -> Option<XY> {
        for (y, xs) in self.cells.iter().enumerate() {
            for (x, cell) in xs.iter().enumerate() {
                if *cell == target {
                    return Some((x, y).into());
                }
            }
        }

        None
    }

    fn neighbors(&self, xy: XY) -> [Option<XY>; 4] {
        let find = |x_offset, y_offset| {
            let new_xy = XY { x: xy.x + x_offset, y: xy.y + y_offset };
            if self.in_bound(new_xy) {
                Some(new_xy)
            } else {
                None
            }
        };

        [
            find(-1, 0),
            find(1, 0),
            find(0, -1),
            find(0, 1),
        ]
    }

    fn in_bound(&self, xy: XY) -> bool {
        xy.x >= 0 && xy.y >= 0
            && self.height > xy.y as usize
            && self.width > xy.x as usize
    }

    fn is_possible(&self, from: XY, to: XY) -> bool {
        self.elevation.is_possible(self.get(from), self.get(to))
    }

    fn get(&self, xy: XY) -> Cell {
        self.cells[xy.y as usize][xy.x as usize]
    }
}

#[derive(Eq, PartialEq)]
struct CostXY(Cost, XY);

impl Ord for CostXY {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}
impl PartialOrd for CostXY {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct BFS<'a> {
    start: XY,
    end: XY,
    grid: &'a Grid,
    frontier: BinaryHeap<CostXY>,
    visits: HashMap<XY, XY>,
    costs: HashMap<XY, Cost>
}

impl<'a> BFS<'a> {
    pub fn new(start: XY, end: XY, grid: &'a Grid) -> Self {
        let mut bfs = Self {
            start,
            end,
            grid,
            frontier: BinaryHeap::new(),
            visits: HashMap::new(),
            costs: HashMap::new(),
        };

        bfs.add(start, start);
        bfs
    }

    fn step_cost(&self, from: XY) -> Cost {
        self.costs.get(&from).unwrap_or(&0) + 1
    }

    fn dist_cost(&self, to: XY) -> Cost {
        ((self.end.x - to.x).abs() + (self.end.y - to.y).abs()) as u64
    }

    fn add(&mut self, from: XY, to: XY) {
        self.visits.insert(to, from);

        let step_cost = self.step_cost(from);
        let dist_cost = self.dist_cost(to);
        self.costs.insert(to, step_cost);
        self.frontier.push(CostXY(step_cost + dist_cost, to));
    }

    pub fn search(&mut self) {
        while let Some(CostXY(_, xy)) = self.frontier.pop() {
            let neighboars = self.grid.neighbors(xy);
            let iter = neighboars
                .iter()
                .filter_map(|xs| *xs);

            let new_cost = self.step_cost(xy);
            for new_xy in iter {
                let old_cost = self.costs.get(&new_xy);

                if self.grid.is_possible(xy, new_xy) && (old_cost.is_none() || *old_cost.unwrap() > new_cost) {
                    self.add(xy, new_xy);
                }
            }
        }
    }

    pub fn backtrack(&self) -> Vec<XY> {
        let mut track = Vec::new();
        let mut cur = self.end;

        while cur != self.start {
            track.push(cur);
            let prev = self.visits.get(&cur).expect("can't calculate track");
            cur = *prev;
        }

        track.reverse();
        track
    }
}
