mod day1;

fn main() {
    match day1::part1::find_max_calories() {
        Ok(max_total) => { println!("Maximum total calories is: {}", max_total); }
        Err(e) => eprintln!("Error: {}", e),
    }
}
