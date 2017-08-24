pub fn main() {
	feature::main();
}

#[cfg(all(feature="winit", feature="glium"))]
mod feature {
	use conrod::{self, widget, Colorable, Positionable, Widget};
	use conrod::backend::glium::glium::{self, Surface};
	use glium::glutin;

	pub fn main() {
		const WIDTH: u32 = 400;
		const HEIGHT: u32 = 200;

		let mut events_loop = glutin::EventsLoop::new();
		let window = glutin::WindowBuilder::new()
			.with_title("Game of Life")
			.with_dimensions(WIDTH, HEIGHT);
		let context = glutin::ContextBuilder::new()
			.with_vsync(true)
			.with_multisampling(4);
		let display = glium::Display::new(window, context, &events_loop).unwrap();

		let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

		widget_ids!(struct Ids { text });
		let Ids = Ids::new(ui.widget_id_generator());

		const FONT_PATH: &'static str =
			concat!(env!("CARGO_MANIFEST_DIR"), "/src/fonts/NotoSans/NotoSans-Regular.ttf");
			ui.fonts.insert_from_file(FONT_PATH).unwrap();

		let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

		let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

		let mut events = Vec::new();

		'render : loop {
			events.clear();

			events_loop.poll_events(|event| { events.push(event); });

			if events.is_empty() {
				events_loop.run_forever(|event| {
					events.push(event);
					glutin::ControlFlow::Break
				});
			}

			for event in events.drain(..) {
				match event.clone() {
					glutin::Event::WindowEvent { event, .. } => {
						match event {
							glutin::WindowEvent::Closed |
							glutin::WindowEvent::KeyboardInput {
								input: glutin::KeyboardInput {
									virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
									..
								},
								..
							} => break 'render,
							_ => (),
						}
					},
					_ => ()
				};

				let input = match conrod::backend::winit::convert_event(event, &display) {
                    None => continue,
                    Some(input) => input,
                };

				ui.handle_event(input);

				let ui = &mut ui.set_widgets();

				widget::Text::new("Hello world")
					.middle_of(ui.window)
					.color(conrod::color::WHITE)
					.font_size(32)
					.set(ids.text, ui)
			}

			if let Some(primitives) = ui.draw_if_changed() {
				renderer.fill(&display, primitives, &image_map);
				let mut target = display.draw();
				target.clear_color(0.0, 0.0, 0.0, 1.0);
				renderer.draw(&display, &mut target, &image_map).unwrap();
				target.finish().unwrap();
			}
		}
	}

	pub fn display(board: &Vec<Vec<bool>>) {
		for y in 0..board.len() {
			let row: &[bool] = &board[y];
			for x in 0..row.len() {
				let is_alive: bool = row[x];

				if is_alive {
					print!("X");
				} else {
					print!(".");
				}
			}
			print!("\n");
		}
		println!("-------");
	}

}

#[cfg(not(all(feature="winit", feature="glium")))]
mod feature {
    pub fn main() {
        println!("This example requires the `winit` and `glium` features. \
                 Try running `cargo run --release --features=\"winit glium\" --example <example_name>`");
    }
}
