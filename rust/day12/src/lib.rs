use std::{collections::{LinkedList, HashMap}, slice::Iter};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct XY {
    pub x: i64, pub y: i64
}

#[wasm_bindgen]
impl XY {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i64, y: i64) -> Self {
        XY { x, y }
    }
}

#[wasm_bindgen]
pub struct Heightmap {
    map: Vec<Vec<char>>,
    elevations: HashMap<char, char>,
}

#[wasm_bindgen]
impl Heightmap {
    #[wasm_bindgen(constructor)]
    pub fn new(map: String) -> Self {
        let heightmap = map
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let mut elevations = HashMap::new();
        elevations.insert('S', 'a');
        elevations.insert('z', 'E');
        let all_chars = ('a'..='z').collect::<Vec<char>>();
        for (i, char) in all_chars.iter().enumerate().skip(1) {
            elevations.insert(all_chars[i - 1], char.clone());
        }

        Heightmap { map: heightmap, elevations }
    }

    #[wasm_bindgen]
    pub fn is_possible_elevation(&self, from: XY, to: XY) -> bool {
        let XY {x, y} = from;
        let XY {x: xn, y: yn} = to;

        if xn < 0 || yn < 0 || yn >= self.map.len() as i64 {
            return false;
        }

        if xn >= self.map[yn as usize].len() as i64 {
            return false;
        }

        let chr = self.map[y as usize][x as usize];
        let chrn = self.map[yn as usize][xn as usize];

        chr == chrn || self.elevations[&chr] == chrn
    }

    #[wasm_bindgen]
    pub fn find_height(&self, name: char) -> Option<XY> {
        for (y, line) in self.map.iter().enumerate() {
            for (x, height) in line.iter().enumerate() {
                if *height == name {
                    return Some(XY::new(x as i64, y as i64));
                }
            }
        }
        return None;
    }
}

static SIDES: [XY; 4] = [XY{ x : -1, y: 0 }, XY { x: 1, y: 0 }, XY { x: 0, y: -1 }, XY { x: 0, y: 1 }];


#[wasm_bindgen]
pub struct BFS {
    heightmap: Heightmap,
    frontier: LinkedList<XY>,
    visited: HashMap<XY, XY>,
    side_iter: Iter<'static, XY>,
    current_frontier: Option<XY>,
}

#[wasm_bindgen]
impl BFS {
    #[wasm_bindgen(constructor)]
    pub fn new(initial: &XY, heightmap: Heightmap) -> Self {
        let mut bfs = Self {
            heightmap,
            frontier: LinkedList::new(),
            visited: HashMap::new(),
            side_iter: SIDES.iter(),
            current_frontier: None,
        };
        bfs.add(*initial, *initial);
        bfs
    }

    fn add(&mut self, from: XY, to: XY) {
        self.visited.insert(to, from);
        self.frontier.push_back(to);
    }

    #[wasm_bindgen]
    pub fn backtrack(&self, xy: &XY) -> Option<XY> {
        self.visited.get(xy).map(|xy| *xy)
    }

    #[wasm_bindgen]
    pub fn next(&mut self) -> Option<XY> {
        if self.current_frontier.is_none() {
            self.current_frontier = self.frontier.pop_front();
        }

        while let Some(xy) = self.current_frontier {
            if let Some(XY { x: x_offset, y: y_offset }) = self.side_iter.next() {
                let XY { x, y } = xy;
                let xyn = XY::new(x + x_offset, y + y_offset);
                if self.visited.contains_key(&xyn) || !self.heightmap.is_possible_elevation(xy, xyn) {
                    continue;
                }

                self.add(xy, xyn);
                return Some(xyn)
            } else {
                self.side_iter = SIDES.iter();
                self.current_frontier = self.frontier.pop_front();
            }
        }

        return None
    }
}
