// --- Part Two ---
// The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

// The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

// In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
// In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
// In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
// Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

// Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;

pub fn find_total_scores() -> Result<i32> {
    let reader = BufReader::new(File::open("src/day2/input.txt")?);
    let mut scores = 0;

    for line in reader.lines() {
        let line = line?;
        let opp = &line[0..1];
        let hero = &line[2..3];

        scores += play(opp, hero);
    }

    Ok(scores)
}

fn play(opp: &str, outcome: &str) -> i32 {
    let mut map = HashMap::new();
    map.insert("A", 1);
    map.insert("B", 2);
    map.insert("C", 3);

    if outcome == "Z" {
        return 6 + (map[opp] % 3) + 1;
    } else if outcome == "Y" {
        return 3 + map[opp];
    } else {
        return (map[opp] + 1) % 3 + 1;
    }
}