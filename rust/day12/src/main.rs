use day12::{BFS, Heightmap};

fn main() {
    let name = std::env::args().nth(1).expect("path is missed");
    let content = std::fs::read_to_string(name).expect("can't read file");

    let heightmap = Heightmap::new(content);
    let start_xy = heightmap.find_height('S').unwrap();
    let end_xy = heightmap.find_height('E').unwrap();

    let mut bfs = BFS::new(start_xy, end_xy, heightmap);
    for _ in &mut bfs {
    }

    println!("{}", bfs.backtrack(end_xy).len());
}
