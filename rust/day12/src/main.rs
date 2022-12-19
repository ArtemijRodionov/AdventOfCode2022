use std::{collections::HashMap};

use day12::{BFS, Heightmap, XY};

fn main() {
    // let name = std::env::args().nth(1).expect("path is missed");
    let name = "../../inputs/12.txt";
    let content = std::fs::read_to_string(name).expect("can't read file");

    let heightmap = Heightmap::new(content);
    let start_xy = heightmap.find_height('S').unwrap();
    let end_xy = heightmap.find_height('E').unwrap();

    let mut bfs = BFS::new(&start_xy, heightmap);
    for xy in &mut bfs {
        println!("{:?}", xy);
        if xy == end_xy {
            break;
        }
    }

    let mut step_count = 0;
    let mut xy = end_xy;
    let visited: HashMap<XY, XY> = bfs.into();
    while let Some(nxy) = visited.get(&xy) {
        step_count += 1;
        if *nxy == start_xy {
            break;
        }
        xy = *nxy;
    }
    println!("{}", step_count);
}
