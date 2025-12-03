use std::{
    fs::File,
    io::{BufReader, Lines},
};

use aoc_2025::read_input_by_line;

fn main() {
    if let Ok(lines) = read_input_by_line("input") {
        part_one(lines);
    }
    if let Ok(lines) = read_input_by_line("input") {
        part_two(lines);
    }
}

fn part_one(lines: Lines<BufReader<File>>) {
    let mut count = 0;
    lines.fold(50, |dial_pos, next_instruction| {
        if let Ok(line) = next_instruction {
            let mut chars = line.chars();
            let direction = chars.next().expect("Invalid input: empty line!");
            let num_clicks = chars
                .collect::<String>()
                .parse::<usize>()
                .expect("Could not parse clicks!")
                % 100;
            let dial_pos = if direction == 'L' {
                if dial_pos < num_clicks {
                    let r = num_clicks - dial_pos;
                    100 - r
                } else {
                    (dial_pos - num_clicks) % 100
                }
            } else {
                (dial_pos + num_clicks) % 100
            };
            if dial_pos == 0 {
                count += 1;
            }
            dial_pos
        } else {
            dial_pos
        }
    });
    println!("Part one: {count}");
}

fn part_two(lines: Lines<BufReader<File>>) {
    let mut count = 0;
    lines.fold(50, |dial_pos, next_instruction| {
        if let Ok(line) = next_instruction {
            let mut chars = line.chars();
            let direction = chars.next().expect("Invalid input: empty line!");
            let mut num_clicks = chars
                .collect::<String>()
                .parse::<usize>()
                .expect("Could not parse clicks!");
            if num_clicks > 99 {
                count += num_clicks / 100;
                num_clicks %= 100;
            }
            let dial_pos = if direction == 'L' {
                if dial_pos < num_clicks {
                    if dial_pos != 0 {
                        count += 1;
                    }
                    let r = num_clicks - dial_pos;
                    100 - r
                } else {
                    (dial_pos - num_clicks) % 100
                }
            } else {
                let mut sum = dial_pos + num_clicks;
                if sum > 99 {
                    sum %= 100;
                    if sum != 0 {
                        count += 1;
                    }
                    sum
                } else {
                    sum
                }
            };
            if dial_pos == 0 {
                count += 1;
            }
            dial_pos
        } else {
            dial_pos
        }
    });
    println!("Part two: {count}");
}
