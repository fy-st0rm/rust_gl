mod window;

use sdl2::{
	event::Event,
	keyboard::Keycode,
};
use std::time::Duration;
use window::Window;

fn main() -> Result<(), String> {
	let mut window = Window::new("Wall Runner", 800, 600)?;

	'running: loop {
		window.clear(0.5, 0.5, 0.5, 0.5);

		for event in window.get_events() {
			match event {
				Event::Quit { .. } => break 'running,
				Event::KeyDown { keycode: Some(keycode), .. } => {
					match keycode {
						Keycode::Escape => break 'running,
						_ => println!("{}", keycode)
					}
				}
				_ => {}
			}
		}

		window.update();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}

	Ok(())
}
