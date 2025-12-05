use aoc_2025::read_input_by_line;
fn main() {
    if let Ok(lines) = read_input_by_line("input") {
        let lines = lines.map(|l| l.expect("invalid input")).collect();
        part_one(&lines);
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
