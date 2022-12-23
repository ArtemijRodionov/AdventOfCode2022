use day12::{BFS, Grid, XY};

fn main() {
    let name = std::env::args().nth(1).expect("path is missed");
    let content = std::fs::read_to_string(name).expect("can't read file");

    let grid = Grid::new(content);
    let start = grid.find('S').expect("can't find start");
    let end = grid.find('E').expect("can't find end");
    let mut bfs = BFS::new(start, end, &grid);
    bfs.search();

    grid.print(bfs.backtrack());
    println!("{}", bfs.backtrack().len() - 2);

    let mut best_bfs = bfs;

    for y in 0..grid.height {
        let candidate = XY{ x: start.x, y: y as i64 };
        let mut bfs = BFS::new(candidate, end, &grid);
        bfs.search();

        let track_length = bfs.backtrack().len();
        if best_bfs.backtrack().len() > track_length {
            best_bfs = bfs;
        }
    }
    let best_start_track = best_bfs.backtrack();
    grid.print(best_start_track);
    println!("{}", best_bfs.backtrack().len() - 2);

}
