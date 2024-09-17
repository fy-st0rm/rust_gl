mod math;
mod utils;
mod window;
mod shader;
mod renderer;
 
use std::time::Duration;
use sdl2::{
	event::Event,
	keyboard::Keycode,
};

use math::vec::*;
use window::Window;
use renderer::*;

fn main() -> Result<(), String> {
	let mut window = Window::new("Wall Runner", 800, 600)?;

	// Creating renderer
	let mut renderer = Renderer::new()?;

	'running: loop {
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

		// Rendering
		renderer.clear(0.5, 0.5, 0.5, 0.5);
		renderer.begin();

		renderer.push_vertex(Vertex {pos: Vec3::new(-0.5, -0.5, 0.0)});
		renderer.push_vertex(Vertex {pos: Vec3::new(0.5,  -0.5, 0.0)});
		renderer.push_vertex(Vertex {pos: Vec3::new(0.0,  0.5,  0.0)});

		renderer.end();

		window.update();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}

	// Cleaning
	renderer.delete();

	Ok(())
}
