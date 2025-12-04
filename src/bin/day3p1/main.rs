const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let mut sum = 0;

    for bank in INPUT.lines() {
        let batteries = bank
            .chars()
            .map(|char| char as u32 - '0' as u32)
            .collect::<Box<[u32]>>();

        println!("Calculating {:?}", batteries);

        let (a, b) = joltage(&batteries);
        let result = batteries[a] * 10 + batteries[b];

        println!("Result {}", result);

        sum += result;
    }

    println!("Sum: {}", sum);
}

fn joltage(batteries: &[u32]) -> (usize, usize) {
    if batteries.len() == 2 {
        return (0, 1);
    }

    let (next_best_a, next_best_b) = joltage(&batteries[1..]);
    let next_best_a = next_best_a + 1;
    let next_best_b = next_best_b + 1;

    if batteries[0] < batteries[next_best_a] {
        return (next_best_a, next_best_b);
    }

    (
        0,
        if batteries[next_best_a] > batteries[next_best_b] {
            next_best_a
        } else {
            next_best_b
        },
    )
}
