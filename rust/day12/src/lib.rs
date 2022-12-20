use std::{collections::{HashMap, BinaryHeap}};

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
#[derive(Clone, Copy)]
#[derive(Hash, PartialEq, Eq, Debug)]
#[derive(Serialize, Deserialize)]
pub struct XY {
    pub x: i32, pub y: i32
}

#[wasm_bindgen]
impl XY {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32) -> Self {
        XY { x, y }
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsArray(pub Vec<XY>);

#[wasm_bindgen]
pub struct Heightmap {
    map: Vec<Vec<char>>,
    elevations: Vec<char>,
}

#[wasm_bindgen]
impl Heightmap {
    #[wasm_bindgen(constructor)]
    pub fn new(map: String) -> Self {
        let map = map
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let mut elevations = Vec::new();
        elevations.push('S');
        elevations.extend('a'..='z');
        elevations.push('E');

        Heightmap { map, elevations }
    }

    fn size(&self) -> usize {
        self.map.len() * self.map[0].len()
    }

    fn is_possible_move(&self, from: XY, to: XY) -> bool {
        let XY {x, y} = from;
        let XY {x: xn, y: yn} = to;

        if xn < 0 || yn < 0 || yn >= self.map.len() as i32 {
            return false;
        }

        if xn >= self.map[yn as usize].len() as i32 {
            return false;
        }

        let get_i = |x, y| {
            let chr = self.map[y as usize][x as usize];
            self.elevations.iter().position(|&x| x == chr).unwrap()
        };

        let chr = get_i(x, y);
        let chrn = get_i(xn, yn);

        return chrn <= chr + 1
    }

    #[wasm_bindgen]
    pub fn find_height(&self, name: char) -> Option<XY> {
        for (y, line) in self.map.iter().enumerate() {
            for (x, height) in line.iter().enumerate() {
                if *height == name {
                    return Some(XY::new(x as i32, y as i32));
                }
            }
        }
        return None;
    }
}

static SIDES: [XY; 4] = [XY{ x : -1, y: 0 }, XY { x: 1, y: 0 }, XY { x: 0, y: -1 }, XY { x: 0, y: 1 }];

pub struct BFS {
    start: XY,
    end: XY,
    heightmap: Heightmap,
    frontier: BinaryHeap<PriorityXY>,
    visits: HashMap<XY, XY>,
    visit_cost: HashMap<XY, i32>
}
#[derive(Eq)]
struct PriorityXY(XY, i32);

impl Ord for PriorityXY {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for PriorityXY {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PriorityXY {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl BFS {
    pub fn new(start: XY, end: XY, heightmap: Heightmap) -> Self {
        let mut bfs = Self {
            start,
            end,
            heightmap,
            frontier: BinaryHeap::new(),
            visits: HashMap::new(),
            visit_cost: HashMap::new(),
        };
        bfs.add(start, start, 0);
        bfs
    }

    fn cost_to_end(&self, xy: XY) -> i32 {
        (xy.x - self.end.x).abs() + (xy.x - self.end.x).abs()
    }

    fn add(&mut self, from: XY, to: XY, cost: i32) {
        self.visit_cost.insert(to, cost);
        self.visits.insert(to, from);
        self.frontier.push(PriorityXY(to, cost + self.cost_to_end(to)));
    }

    pub fn backtrack(&self, xy: XY) -> Vec<XY> {
        let mut cur = Some(xy);
        let mut path = Vec::new();
        while let Some(xy) = cur {
            if path.len() > self.heightmap.size() {
                panic!("Path is too long")
            }

            if xy == self.start {
                break
            }

            path.push(xy);
            cur = self.visits.get(&xy).map(|xy| *xy);
        }

        path.reverse();
        path
    }

    pub fn step(&mut self) -> Option<XY> {
        self.frontier
            .pop()
            .map(|PriorityXY(xy, _)| {
                if xy == self.end {
                    return xy;
                }

                for XY { x: x_offset, y: y_offset } in SIDES.iter() {
                    let XY { x, y } = xy;
                    let xyn = XY::new(x + x_offset, y + y_offset);

                    if !self.heightmap.is_possible_move(xy, xyn) {
                        continue;
                    }

                    let new_cost = 1 + self.visit_cost.get(&xy).unwrap();
                    if self.visit_cost
                        .get(&xyn)
                        .map(|&old_cost| new_cost < old_cost)
                        .unwrap_or(true)
                    {
                        self.add(xy, xyn, new_cost);
                    }
                }

                return xy;
        })
    }
}

#[wasm_bindgen]
struct WasmBFS(BFS);

#[wasm_bindgen]
impl WasmBFS {

    #[wasm_bindgen(constructor)]
    pub fn new(start: &XY, end: &XY, heightmap: Heightmap) -> Self {
        WasmBFS(BFS::new(*start, *end, heightmap))
    }

    #[wasm_bindgen]
    pub fn step(&mut self) -> Option<XY> {
        self.0.step()
    }

    #[wasm_bindgen]
    pub fn backtrack(&self, xy: &XY) -> JsValue {
        let xs = JsArray(self.0.backtrack(*xy));
        JsValue::from_serde(&xs).unwrap()
    }

    #[wasm_bindgen]
    pub fn search(&mut self) {
        for _ in &mut self.0 {}
    }
}

impl Iterator for BFS {
    type Item = XY;

    fn next(&mut self) -> Option<Self::Item> {
        self.step()
    }
}
