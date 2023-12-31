// mod day1;
mod day2;

fn main() {
    // match day1::part1::find_max_calories() {
    //     Ok(max_total) => { println!("Maximum total calories is: {}", max_total); }
    //     Err(e) => eprintln!("Error: {}", e),
    // }

    // match day1::part2::find_top_three_calories_sum() {
    //     Ok(top_three_sum) => { println!("Sum of top three calories is: {}", top_three_sum); }
    //     Err(e) => eprintln!("Error: {}", e),
    // }

    // match day2::part1::find_total_scores() {
    //     Ok(scores) => { println!("Total scores equals: {}", scores); }
    //     Err(e) => eprintln!("Error: {}", e),
    // }

    match day2::part2::find_total_scores() {
        Ok(scores) => { println!("Total scores equals: {}", scores); }
        Err(e) => eprintln!("Error: {}", e),
    }
}
