#![feature(core)]

extern crate rustbox;

use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

mod view;
mod model;

use std::{thread, time};
use view;
use model::{step};

fn main() {
	view::init();
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
		view::poll_events();
		thread::sleep(delay);
		board = step(&board);
	}
}
