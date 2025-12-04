const INPUT: &'static str = include_str!("input.txt");

const BATTERY_COUNT: usize = 12;

fn main() {
    let mut sum = 0;

    for bank in INPUT.lines() {
        let batteries = bank.chars().collect::<Box<[_]>>();

        let selected = joltage(&batteries);

        let selected = selected
            .into_iter()
            .map(|i| batteries[i])
            .collect::<String>();

        sum += dbg!(selected).parse::<u64>().unwrap();
    }

    println!("Sum: {}", sum);
}

fn joltage(batteries: &[char]) -> Box<[usize]> {
    if batteries.len() == BATTERY_COUNT {
        return (0..BATTERY_COUNT).into_iter().collect::<Box<[_]>>();
    }

    let mut selected = joltage(&batteries[1..]);
    for i in &mut selected {
        *i += 1;
    }

    println!("Step {:?}", batteries);
    println!("From {}", create_string(batteries, &selected));

    // if the added character is bigger than the first selected charact a bigger number can be created
    if batteries[0] >= batteries[selected[0]] {
        // Swap the first element with the smallest element

        let mut smallest = 0;
        for i in 1..BATTERY_COUNT {
            if batteries[selected[i]] < batteries[selected[smallest]] {
                smallest = i;
            }
        }

        selected[0..=smallest].rotate_right(1);
        selected[0] = 0;
        println!("Replaced {}", smallest);
    }

    println!("Result {}", create_string(batteries, &selected));

    selected
}

fn create_string(chars: &[char], indices: &[usize]) -> String {
    indices.into_iter().map(|&i| chars[i]).collect()
}
