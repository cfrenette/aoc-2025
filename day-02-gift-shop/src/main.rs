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
    println!(
        "Part one: {}",
        invalid_ids.iter().fold(0, |acc, num| acc + num)
    );
}
