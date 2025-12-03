const INPUT: &'static str = include_str!("input.txt");

fn main() {
    for range in INPUT.split(',') {
        let mut range = range.trim().split('-').take(2);

        let (Some(start), Some(end)) = (range.next(), range.next()) else {
            continue;
        };

        let start: u32 = start.parse().unwrap();
        let end: u32 = end.parse().unwrap();
    }
}
