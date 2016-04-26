extern crate rand;

use rand::distributions::IndependentSample;
use rand::distributions::Range;

use std::io::{self, Read, Write};

fn main() {
    pipe(&mut io::stdin(), &mut io::stdout())
}

fn pipe(input: &mut Read, output: &mut Write) {
    let sample_range = Range::new(0, 100000);
    let mut rng = rand::thread_rng();

    for b in input.bytes() {
        let mut b = b.expect("Failed to read");

        let r = sample_range.ind_sample(&mut rng);
        if r < 1 {
            // flip a bit
            b ^= 0b00010000
        }

        output.write(&[b]).expect("Failed to write");
    }
}
