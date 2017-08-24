#[cfg(all(feature="winit", feature="glium"))] #[macro_use] extern crate conrod;

mod view;
mod model;

use std::{thread, time};
use view::main as display;
use model::{step};

fn main() {
	let delay: time::Duration = time::Duration::from_secs(1);
	let mut board = vec![
		vec![false, true, false, false, false, false, false],
		vec![false, false, true, false, false, false, false],
		vec![true, true, true, false, false, false, false],
		vec![false, false, false, false, false, false, false],
		vec![false, false, false, false, false, false, false],
		vec![false, false, false, false, false, false, false],
		vec![false, false, false, false, false, false, false],
	];
	loop {
		display();
		thread::sleep(delay);
		board = step(&board);
	}
}
