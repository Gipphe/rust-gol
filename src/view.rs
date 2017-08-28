let mut rustbox;
pub fn init() {
	rustbox = match RustBox::init(Default::default()) {
		Result::Ok(v) => v,
		Result::Err(e) => panic!("{}", e),
	};
	rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Hello world");
	rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black, "Press 'q' to quit");
	rustbox.present();
}
pub fn poll_events() {
	match rustbox.poll_events(false) {
		Ok(rustbox::Event::KeyEvent(key)) => {
			match key {
				Key::Char('q') => { break; },
				_ => (),
			}
		},
		Err(e) => panic!("{}", e.description()),
		_ => ()
	}
}

