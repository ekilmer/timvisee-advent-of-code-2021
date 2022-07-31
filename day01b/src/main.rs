pub fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u16>>()
            .windows(4)
            .filter(|window| window[0] < window[3])
            .count(),
    );
}
