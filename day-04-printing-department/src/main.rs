use aoc_2025::read_input_by_line;

fn main() {
    if let Ok(mut lines) = read_input_by_line("example") {
        let size = lines.size_hint().1.unwrap_or(lines.size_hint().0);
        let mut map = Vec::with_capacity(size);
        while let Some(Ok(line)) = lines.next() {
            map.push(line.chars().map(|c| c == '@').collect());
        }
        part_one(&map);
    }
}

fn part_one(map: &Vec<Vec<bool>>) {
    let (rows, cols) = (map.len(), map.first().unwrap().len());
    let mut num_free_positions = Vec::with_capacity(rows);
    // Initialize, setting the free positions due to the borders
    let mut first = vec![3u8; cols];
    first[0] = 5;
    first[cols - 1] = 5;
    let last = first.clone();
    num_free_positions.push(first);
    for _row_num in 1..rows - 1 {
        let mut row = vec![0; cols];
        row[0] = 3;
        row[cols - 1] = 3;
        num_free_positions.push(row);
    }
    num_free_positions.push(last);

    for (row_num, row) in map.iter().enumerate() {
        for (col_num, col) in row.iter().enumerate() {
            // If the position does not contain a paper roll,
            // add a free position to each of its neighbors
            if !col {
                if row_num > 0 {
                    if col_num > 0 {
                        num_free_positions[row_num - 1][col_num - 1] += 1;
                    }
                    num_free_positions[row_num - 1][col_num] += 1;
                    if col_num < cols - 1 {
                        num_free_positions[row_num - 1][col_num + 1] += 1;
                    }
                }
                if row_num < rows - 1 {
                    if col_num > 0 {
                        num_free_positions[row_num + 1][col_num - 1] += 1;
                    }
                    num_free_positions[row_num + 1][col_num] += 1;
                    if col_num < cols - 1 {
                        num_free_positions[row_num + 1][col_num + 1] += 1;
                    }
                }
                if col_num > 0 {
                    num_free_positions[row_num][col_num - 1] += 1;
                }
                if col_num < cols - 1 {
                    num_free_positions[row_num][col_num + 1] += 1;
                }
            }
        }
    }

    let mut can_be_accessed = 0;

    // count the number of rolls with > 4 free spots
    for (row_num, row) in map.iter().enumerate() {
        for (col_num, col) in row.iter().enumerate() {
            if *col {
                if num_free_positions[row_num][col_num] > 4 {
                    can_be_accessed += 1;
                }
            }
        }
    }

    println!("Part one: {}", can_be_accessed);
}
