use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn find_top_three_calories_sum() -> io::Result<i32> {
    let file = File::open("src/day1/input.txt")?;
    let reader = BufReader::new(file);

    let mut heap = LimitedHeap::<Reverse<i32>>::new(3);
    let mut sum: i32 = 0;

    for line in reader.lines() {
        let line = line?;
        if !line.is_empty() {
            sum += line.parse::<i32>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse line"))?;
        } else {
            heap.push(Reverse(sum));
            sum = 0;
        }
    }

    heap.push(Reverse(sum));

    Ok(heap.heap.drain().map(|Reverse(value)| value).sum::<i32>())
}

pub struct LimitedHeap<T: Ord> {
    heap: BinaryHeap<T>,
    size: usize,
}

impl<T: Ord> LimitedHeap<T> {
    pub fn new(size: usize) -> Self {
        LimitedHeap {
            heap: BinaryHeap::with_capacity(size),
            size: size,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.heap.len() < self.size {
            self.heap.push(value);
        } else {
            if value < *self.heap.peek().unwrap() {
                self.heap.pop();
                self.heap.push(value);
            }
        }
    }
}