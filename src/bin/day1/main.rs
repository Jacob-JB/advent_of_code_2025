const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let mut dial = 50u32;
    let mut count = 0u32;

    for line in INPUT.lines() {
        let Some(direction) = line.chars().nth(0) else {
            continue;
        };

        let Some(amount) = line.get(1..) else {
            continue;
        };

        let amount: u32 = amount.parse().unwrap();

        let reflected = match direction {
            'R' => false,
            'L' => true,
            _ => continue,
        };

        if reflected {
            dial = 100 - dial;

            if dial == 100 {
                dial = 0;
            }
        }

        // Dial is now always being turned right.

        dial += amount;

        count += dial / 100;
        dial = dial % 100;

        if reflected {
            dial = 100 - dial;

            if dial == 100 {
                dial = 0;
            }
        }

        dbg!(dial);
    }

    println!("{}", count);
}
