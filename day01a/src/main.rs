#![cfg_attr(feature = "unstable-features", feature(array_windows))]

pub fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u16>>();

    let filtered_input = {
        cfg_if::cfg_if! {
            if #[cfg(feature = "unstable-features")] {
                input.array_windows().filter(|[a, b]| a < b)
            } else {
                input.windows(2).filter(|window| window[0] < window[1])
            }
        }
    };

    println!("{}", filtered_input.count());
}
