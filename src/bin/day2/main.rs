const INPUT: &'static str = include_str!("input.txt");
// const INPUT: &'static str = include_str!("input_example.txt");

fn main() {
    let mut sum = 0u64;

    for range in INPUT.split(',') {
        let mut range = range.trim().split('-').take(2);

        let (Some(start), Some(end)) = (range.next(), range.next()) else {
            continue;
        };

        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();

        let mut current = start - 1;

        loop {
            current = next_id(current);

            if current > end {
                println!("{} too big", current);
                break;
            }

            println!("{} found", current);
            sum += current;
        }
    }

    println!("Sum: {}", sum);
}

fn next_id(previous: u64) -> u64 {
    let string = previous.to_string();

    let digits = string.chars().count();

    if digits % 2 == 1 {
        // If there is an odd number of digits, the next output is the lowest id with the next number of digits
        10u64.pow(digits as u32) + 10u64.pow(digits as u32 / 2)
    } else {
        let front: u64 = string.get(0..digits / 2).unwrap().parse().unwrap();
        let back: u64 = string.get(digits / 2..digits).unwrap().parse().unwrap();

        match front.cmp(&back) {
            std::cmp::Ordering::Less => (front + 1).to_string() + &(front + 1).to_string(),
            std::cmp::Ordering::Equal => (front + 1).to_string() + &(front + 1).to_string(),
            std::cmp::Ordering::Greater => front.to_string() + &front.to_string(),
        }
        .parse()
        .unwrap()
    }
}

// 199 617
// 200 200
// the back is larger than the front, bump the front by one and duplicate it to the back
//
// 200 200
// 201 201
// they are the same, bump them both
//
// 617 199
// 617 617
// the front is larger than the back, duplicate it to the back
