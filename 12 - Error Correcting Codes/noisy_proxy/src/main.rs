#[macro_use]
extern crate conrod;
extern crate find_folder;
extern crate piston_window;
extern crate rand;

mod gui;

use rand::distributions::IndependentSample;
use rand::distributions::Range;

use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let slider_value = Arc::new(Mutex::new(0.0));
    let slider_value_clone = slider_value.clone();
    thread::spawn(|| {
        pipe(&mut io::stdin(), &mut io::stdout(), slider_value_clone)
    });

    gui::launch(slider_value);
}

fn pipe(input: &mut Read, output: &mut Write, slider_value: Arc<Mutex<f64>>) {
    let sample_range = Range::new(0.0, 100.0);
    let mut rng = rand::thread_rng();

    for b in input.bytes() {
        let mut b = b.expect("Failed to read");

        let r = sample_range.ind_sample(&mut rng);
        if r < *slider_value.lock().expect("Failed to sync slider value") {
            // flip a bit
            b ^= 0b00010000
        }

        output.write(&[b]).expect("Failed to write");
    }
}
