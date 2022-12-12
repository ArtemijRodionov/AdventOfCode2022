fn main() {
    // let name = std::env::args().nth(1).expect("path is missed");
    let name = "../../inputs/12.txt";
    let content = std::fs::read_to_string(name).expect("can't read file");
    let heightmap = content
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let get_xy = || {
        for (y, line) in heightmap.iter().enumerate() {
            for (x, char) in line.iter().enumerate() {
                if char == &'S' {
                    return Some((x as i64, y as i64));
                }
            }
        }
        return None;
    };
    let start_xy = get_xy().unwrap();

    let mut possible_elevation = std::collections::HashMap::new();
    possible_elevation.insert('S', 'a');
    possible_elevation.insert('z', 'E');
    let all_chars = ('a'..='z').collect::<Vec<char>>();
    for (i, char) in all_chars.iter().enumerate().skip(1) {
        possible_elevation.insert(all_chars[i - 1], char.clone());
    }

    let mut frontier = std::collections::LinkedList::new();
    let mut path_to_exit = std::collections::HashMap::new();
    frontier.push_back(start_xy);

    let mut populate_path_to_exit = || {
        while let Some((x, y)) = frontier.pop_front() {
            let chr = heightmap[y as usize][x as usize];
            if chr == 'E' {
                return Some((x, y));
            }
            for (x_offset, y_offset) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (xn, yn) = (x + x_offset, y + y_offset);
                if xn < 0 || yn < 0 || yn >= heightmap.len() as i64 {
                    continue;
                }
                let line = &heightmap[yn as usize];
                if xn >= line.len() as i64 {
                    continue;
                }

                let chrn = heightmap[yn as usize][xn as usize];
                println!("{}->{} {}:{}->{}:{}", chr, chrn, x, y, xn, yn);
                if path_to_exit.contains_key(&(xn, yn))  { continue; }
                if chr == chrn || possible_elevation[&chr] == chrn {
                    path_to_exit.insert((xn, yn), (x, y));
                    frontier.push_back((xn, yn));
                }
            }
        }
        return None
    };
    let mut step_count = 0;
    let mut xy = populate_path_to_exit().unwrap();
    while let Some(nxy) = path_to_exit.get(&xy) {
        step_count += 1;
        if *nxy == start_xy {
            break;
        }
        xy = *nxy;
    }
    println!("{}", step_count);
}
