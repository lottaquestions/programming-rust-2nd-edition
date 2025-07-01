// Number generator
struct NumGen {
    curremt: usize,
}

// The number generator will start from 2
impl NumGen {
    fn new() -> Self {
        NumGen { curremt: 2 }
    }
}

impl Iterator for NumGen {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.curremt;
        self.curremt += 1;
        Some(val)
    }
}





fn main() {
    let mut next_number = NumGen::new();
    // Find the square root of the first perfect square in the series
    let sqrt = 'outer: loop {
        let n = next_number.next().unwrap();
        for i in 1.. {
            let square = i * i;
            if square == n {
                // Found a square root
                break 'outer i;
            }
            if square > n {
                // `n` isn't a perfect square, try the next
                break;
            }
        }
    };

    println!("Found a square root: {}", sqrt);
}
