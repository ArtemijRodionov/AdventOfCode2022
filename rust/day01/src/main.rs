

fn main() {
    let file_path = std::env::args().nth(1).expect("specify the path as the first argument");
    let input = std::fs::read_to_string(file_path).unwrap();
    let lines: Vec<_> = input
        .lines()
        .map(|x| x.parse::<u64>().ok())
        .collect();
    let mut packs = lines
        .split(Option::is_none)
        .map(|xs| xs.iter().map(|v| v.unwrap()).sum::<u64>())
        .collect::<Vec<_>>();
    packs.sort();

    println!("{:?}", packs[0]);
    println!("{:?}", packs.iter().rev().take(3).sum::<u64>());
}
