use std::sync::{Arc, Mutex};
use piston_window::*;

pub fn new() -> Box<Fn()> {
    let mut window: Arc<Mutex<PistonWindow>> = Arc::new(Mutex::new(
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap()));

	Box::new(move || {
		while let Some(event) = window.next() {
			window.draw_2d(&event, |context, graphics| {
				clear([1.0; 4], graphics);
				rectangle([1.0, 0.0, 0.0, 1.0], // red
						[0.0, 0.0, 100.0, 100.0],
						context.transform,
						graphics);
			});
		}
	})
}
