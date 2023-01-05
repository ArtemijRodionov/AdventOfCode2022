use std::{collections::HashSet};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
enum PossibleMove {
    Bottom, BottomLeft, BottomRight
}

impl PossibleMove {
    fn start() -> Self {
        PossibleMove::Bottom
    }

    fn next(self) -> Option<Self> {
        match self {
            PossibleMove::Bottom => Some(PossibleMove::BottomLeft),
            PossibleMove::BottomLeft => Some(PossibleMove::BottomRight),
            _ => None
        }
    }

    fn shift(self, cell: Cell) -> Cell {
        match self {
            PossibleMove::Bottom => Cell { x: cell.x, y: cell.y + 1 },
            PossibleMove::BottomLeft => Cell { x: cell.x - 1, y: cell.y + 1 },
            PossibleMove::BottomRight => Cell { x: cell.x + 1, y: cell.y + 1 },
        }
    }
}

struct PossibleCells {
    inner: Cell,
    possible_move: Option<PossibleMove>
}

impl PossibleCells {
    fn new(cell: Cell) -> Self {
        PossibleCells { inner: cell, possible_move: Some(PossibleMove::start()) }
    }
}

impl Iterator for PossibleCells {
    type Item = Cell;

    fn next(&mut self) -> Option<Cell> {
        if let Some(next_move) = self.possible_move {
            self.possible_move = next_move.next();
            return Some(next_move.shift(self.inner))
        }

        None
    }
}

#[derive(Clone)]
pub struct Grid {
    pub cells: HashSet<Cell>
}

pub struct Sides {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Grid {

    pub fn new_simulation(&self, hole: Cell) -> Simulation {
        Simulation::new(self.clone(), hole)
    }

    pub fn sides(&self) -> Sides {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        for cell in &self.cells {
            min_x = cell.x.min(min_x);
            max_x = cell.x.max(max_x);
            min_y = cell.y.min(min_y);
            max_y = cell.y.max(max_y);
        }

        Sides { left: min_x, right: max_x, top: min_y, bottom: max_y }
    }

    pub fn add(&mut self, cell: Cell) {
        self.cells.insert(cell);
    }

    fn contains(&self, cell: &Cell) -> bool {
        self.cells.contains(cell)
    }
}

pub struct Simulation {
    pub grid: Grid,
    pub falling: Option<Cell>,
    pub hole: Cell,
}

impl Simulation {
    pub fn new(grid: Grid, hole: Cell) -> Self {
        Self { grid, hole, falling: None }
    }

    pub fn fall(&mut self) {
        if let Some(falling) = self.falling {
            self.falling = None;
            for possible_cell in PossibleCells::new(falling) {
                if !self.grid.contains(&possible_cell) {
                    self.falling = Some(possible_cell);
                    break;
                }
            }

            if self.falling == None {
                self.grid.add(falling);
            }
        } else {
            if !self.grid.contains(&self.hole) {
                self.falling = Some(self.hole);
            }
        }
    }
}

pub fn get_rocks() -> HashSet<Cell> {
    let mut cells = std::collections::HashSet::new();
    let input_path = std::env::args().nth(1).unwrap_or(
        "input.txt".into()
    );
    let input = std::fs::read_to_string(input_path).expect("Can't read file");

    for structure in input.split('\n') {
        if structure.is_empty() {
            continue;
        }

        let mut prev = None;
        for rock in structure.split(" -> ") {
            let (x, y) = {
                let mut xy = rock.split(',');
                let mut get = || {
                    xy
                        .next()
                        .expect(format!(
                            "can't get next position: '{}'", structure
                        ).as_str())
                        .parse()
                        .expect(format!(
                            "can't parse position: '{}'", structure
                        ).as_str())
                };

                (get(), get())
            };

            if let Some((px, py)) = prev {
                use std::cmp::{min, max};
                let (from_x, to_x) = (min(px, x), max(px, x));
                let (from_y, to_y) = (min(py, y), max(py, y));

                for rock_x in from_x..=to_x {
                    for rock_y in from_y..=to_y {
                        cells.insert((rock_x, rock_y));
                    }
                }
            }

            prev = Some((x, y));
        }
    }

    cells
        .into_iter()
        .map(|(x, y)| Cell { x, y })
        .collect()
}
