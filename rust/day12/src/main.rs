use std::{collections::{LinkedList, HashMap}, slice::Iter};

type XY = (i64, i64);

struct Heightmap {
    map: Vec<Vec<char>>,
    elevations: HashMap<char, char>,
}

impl Heightmap {
    fn new(map: String) -> Self {
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

    fn is_possible_elevation(&self, from: XY, to: XY) -> bool {
        let (x, y) = from;
        let (xn, yn) = to;

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

    fn find_height(&self, name: &char) -> Option<XY> {
        for (y, line) in self.map.iter().enumerate() {
            for (x, char) in line.iter().enumerate() {
                if char == name {
                    return Some((x as i64, y as i64));
                }
            }
        }
        return None;
    }
}

static SIDES: [XY; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];


struct BFS<'a> {
    heightmap: &'a Heightmap,
    frontier: LinkedList<XY>,
    visited: HashMap<XY, XY>,
    side_iter: Iter<'a, XY>,
    current_frontier: Option<XY>,
}

impl<'a> BFS<'a> {
    fn new(initial: XY, heightmap: &'a Heightmap) -> Self {
        let mut bfs = Self {
            heightmap,
            frontier: LinkedList::new(),
            visited: HashMap::new(),
            side_iter: SIDES.iter(),
            current_frontier: None,
        };
        bfs.add(initial, initial);
        bfs
    }

    fn add(&mut self, from: XY, to: XY) {
        self.visited.insert(to, from);
        self.frontier.push_back(to);
    }
}

impl<'a> Iterator for BFS<'a> {
    type Item = XY;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_frontier.is_none() {
            self.current_frontier = self.frontier.pop_front();
        }

        while let Some(xy) = self.current_frontier {
            if let Some((x_offset, y_offset)) = self.side_iter.next() {
                let (x, y) = xy;
                let xyn = (x + x_offset, y + y_offset);
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




fn main() {
    let name = std::env::args().nth(1).expect("path is missed");
    // let name = "../../inputs/12.txt";
    let content = std::fs::read_to_string(name).expect("can't read file");

    let heightmap = Heightmap::new(content);
    let start_xy = heightmap.find_height(&'S').unwrap();
    let end_xy = heightmap.find_height(&'E').unwrap();

    let mut bfs = BFS::new(start_xy, &heightmap);
    for xy in &mut bfs {
        if xy == end_xy {
            break;
        }
    }

    let mut step_count = 0;
    let mut xy = end_xy;
    while let Some(nxy) = bfs.visited.get(&xy) {
        step_count += 1;
        if *nxy == start_xy {
            break;
        }
        xy = *nxy;
    }
    println!("{}", step_count);
}
