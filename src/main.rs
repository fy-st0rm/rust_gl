mod window;
mod shader;

use std::{
	ptr,
	time::Duration,
};
use sdl2::{
	event::Event,
	keyboard::Keycode,
};
use gl;
use gl::types::*;
use window::Window;
use shader::Shader;

// Shaders
const v_shader_src: &str = r#"
	#version 440 core
	layout (location = 0) in vec3 pos;
	void main() {
		gl_Position = vec4(pos, 1.0);
	}
"#;

const f_shader_src: &str = r#"
	#version 440 core
	layout (location = 0) out vec4 color;
	void main() {
		color = vec4(1, 0, 0, 1.0);
	}
"#;

fn main() -> Result<(), String> {
	let mut window = Window::new("Wall Runner", 800, 600)?;

	unsafe {
		gl::Enable(gl::DEPTH_TEST);
	}

	// Setting up buffers
	let vertices: [f32; 9] = [
		-0.2, -0.2, 0.0,
		 0.2, -0.2, 0.0,
		 0.0,  0.2, 0.0,
	];
	let mut vbo = 0;
	let mut vao = 0;

	unsafe {
		gl::GenVertexArrays(1, &mut vao);
		gl::BindVertexArray(vao);

		gl::GenBuffers(1, &mut vbo);
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(
			gl::ARRAY_BUFFER,
			(vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
			vertices.as_ptr() as *const _,
			gl::STATIC_DRAW
		);

		gl::VertexAttribPointer(
			0, 3, gl::FLOAT, gl::FALSE,
			3 * std::mem::size_of::<GLfloat>() as GLsizei,
			ptr::null()
		);
		gl::EnableVertexAttribArray(0);

		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		gl::BindVertexArray(0);
	}

	// Creating shader
	let shader = Shader::from_src(v_shader_src, f_shader_src).unwrap();

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

		// Rendering
		unsafe {
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
			shader.bind();
			gl::BindVertexArray(vao);
			gl::DrawArrays(gl::TRIANGLES, 0, 3);
		}

		window.update();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}

	// Cleaning
	unsafe {
		gl::DeleteVertexArrays(1, &vao);
		shader.delete();
	}

	Ok(())
}
