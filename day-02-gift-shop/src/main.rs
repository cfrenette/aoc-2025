use std::collections::HashSet;

use aoc_2025::read_input_to_string;

fn main() {
    let mut input = String::new();
    read_input_to_string("input", &mut input).unwrap();
    let mut ranges = Vec::new();
    let mut input = input.split(',');
    while let Some(range_string) = input.next() {
        if let Some((start, end)) = range_string.split_once('-') {
            let range = (start.trim(), end.trim());
            ranges.push(range);
        }
    }
    part_one(&ranges);
    part_two(&ranges);
}
fn part_one(ranges: &Vec<(&str, &str)>) {
    let mut invalid_ids = Vec::new();

    for range in ranges {
        let mut start = range.0;
        let mut end = range.1;

        // skip if all numbers in range are odd length
        if start.len() == end.len() && start.len() % 2 != 0 {
            continue;
        }

        // handle special case where start or end are odd by
        // overwriting with the closest even length sequence
        let mut even_start = String::from("1");
        if start.len() % 2 != 0 {
            for _ in 0..start.len() {
                even_start += "0";
            }
            start = &even_start;
        }
        let mut even_end = String::from("9");
        if end.len() % 2 != 0 {
            for _ in 0..end.len() - 2 {
                even_end += "9";
            }
            end = &even_end;
        }

        // skip if modified start/end no longer make sense
        if start >= end {
            continue;
        }

        // Invalid ID patterns are just the first half repeated
        // take the first half of the start of the range, check
        // to see if the first half repeated is still in range,
        // increment, try again until out of range
        let lower_limit = start
            .parse::<usize>()
            .expect(&format!("start of range parse error: {start}"));
        let upper_limit = end
            .parse::<usize>()
            .expect(&format!("end of range parse error: {end}"));

        let (first_half, _) = start.split_at(start.len() / 2);
        let num_as_string = String::from(first_half) + first_half;
        let mut num = num_as_string
            .parse::<usize>()
            .expect(&format!("start of range parse error: {start}"));
        let mut first_half_as_num = first_half
            .parse::<usize>()
            .expect("error parsing first half");

        while num <= upper_limit {
            if num >= lower_limit {
                invalid_ids.push(num);
            }
            first_half_as_num += 1;
            num = (first_half_as_num.to_string() + &first_half_as_num.to_string())
                .parse()
                .expect("error parsing num");
        }
    }
    println!("Part one: {}", invalid_ids.iter().sum::<usize>());
}

fn part_two(ranges: &Vec<(&str, &str)>) {
    let mut invalid_ids = Vec::new();

    for range in ranges {
        let (start, end) = range;

        invalid_ids.append(&mut generate_patterns_between(start, end));
    }
    println!("Part two: {}", invalid_ids.iter().sum::<usize>());
}

fn generate_patterns_between(start: &str, end: &str) -> Vec<usize> {
    let lower_limit = start.parse::<usize>().expect("could not parse lower limit");
    let upper_limit = end.parse::<usize>().expect("could not parse upper limit");
    let num_chars_lower = start.len();
    let num_chars_upper = end.len();

    let mut patterns = HashSet::new();

    for num_chars in num_chars_lower..=num_chars_upper {
        // needs to repeat at least twice
        for repeat_len in 1..=(num_chars / 2) {
            // only generate patterns that could repeat cleanly
            if num_chars % repeat_len == 0 {
                let start = String::from("1") + &(String::from("0").repeat(repeat_len - 1));
                let start: usize = start.parse().expect("invalid start generated");

                let end = String::from("9") + &(String::from("9").repeat(repeat_len - 1));
                let end = end.parse().expect("invalid end generated");

                // only slightly better than brute-force, I'm tired and it's bedtime :(
                for n in start..=end {
                    let num = n
                        .to_string()
                        .repeat(num_chars / repeat_len)
                        .parse()
                        .expect("invalid num generated");
                    if lower_limit <= num && num <= upper_limit {
                        patterns.insert(num);
                    }
                }
            }
        }
    }
    patterns.drain().collect()
}
