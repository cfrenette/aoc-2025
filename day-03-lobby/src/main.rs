use aoc_2025::read_input_by_line;
fn main() {
    if let Ok(lines) = read_input_by_line("input") {
        let lines = lines.map(|l| l.expect("invalid input")).collect();
        part_one(&lines);
        part_two(&lines);
    }
}
fn part_one(lines: &Vec<String>) {
    let mut sum = 0;
    for line in lines {
        let line = line
            .chars()
            .map(|c| c.to_digit(10).expect("invalid digit"))
            .collect::<Vec<u32>>();

        // start with the left and right pointers at the beginning
        let (mut left, mut right) = (0, 1);

        // remember the position of the maximal ones digit
        let mut max_pos = right;

        while right < line.len() - 1 {
            // if left < right and right is not the last, set left = right
            // and reset the maximum
            if line.get(left).unwrap() < line.get(right).unwrap() {
                left = right;
                right = left + 1;
                max_pos = right;
            } else {
                right += 1;
                if line.get(max_pos).unwrap() < line.get(right).unwrap() {
                    max_pos = right;
                }
            }
        }
        sum += line.get(left).unwrap() * 10 + line.get(max_pos).unwrap();
    }
    println!("Part one: {sum}");
}

fn part_two(lines: &Vec<String>) {
    let mut sum = 0;
    for line in lines {
        let line = line
            .chars()
            .map(|c| c.to_digit(10).expect("invalid digit"))
            .collect::<Vec<u32>>();

        let mut leftmost = 0;
        // running total of digit values
        let mut place_value = 0;

        // loop through each position starting with the rightmost
        // and slide the window right, swapping digits right to left
        // if a new maximum is discovered
        for digit in (0..12).rev() {
            let rightmost = line.len() - digit;
            let sliding_window = &line[leftmost..rightmost];
            let (mut max_pos, mut max_val) = (0, 0);

            for (pos, &val) in sliding_window.iter().enumerate() {
                if val > max_val {
                    max_pos = pos;
                    max_val = val;
                }
            }

            // multiply prior by 10 (increase place value)
            place_value = place_value * 10 + u64::from(max_val);
            leftmost += max_pos + 1;
        }
        sum += place_value;
    }
    println!("Part two: {sum}");
}
