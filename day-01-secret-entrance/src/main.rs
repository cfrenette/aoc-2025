use std::{
    fs::File,
    io::{BufReader, Lines},
};

use aoc_2025::read_input_by_line;

fn main() {
    if let Ok(lines) = read_input_by_line("input") {
        part_one(lines);
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
