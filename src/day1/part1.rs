use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn find_max_calories() -> io::Result<i32> {
    let reader = BufReader::new(File::open("src/day1/input.txt")?);
    let mut max_total: i32 = 0;
    let mut sum: i32 = 0;

    for line in reader.lines() {
        let line = line?;
        if !line.is_empty() {
            sum += line.parse::<i32>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse line"))?;
        } else {
            max_total = max_total.max(sum);
            sum = 0;
        }
    }

    max_total = max_total.max(sum);

    Ok(max_total)
}