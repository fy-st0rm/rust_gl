mod window;

use std::{
	ffi::{ CStr, CString },
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

fn compile_shader(src: &str, shader_type: GLenum) -> GLuint {
	let shader;
	unsafe {
		shader = gl::CreateShader(shader_type);
		let c_str = CString::new(src.as_bytes()).unwrap();
		gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
		gl::CompileShader(shader);

		// Check for error
		let mut sucess = gl::FALSE as GLint;
		let mut info_log = vec![0; 512];
		gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut sucess);
		if sucess != gl::TRUE as GLint {
			gl::GetShaderInfoLog(
				shader, 512, ptr::null_mut(),
				info_log.as_mut_ptr() as *mut GLchar
			);
			panic!(
				"ERROR::SHADER::COMPILATION_FAILED\n{}",
				CStr::from_ptr(info_log.as_ptr() as *const i8).to_str().unwrap()
			);
		}
	}
	shader
}

fn link_program(v_shader: GLuint, f_shader: GLuint) -> GLuint {
	let program;
	unsafe {
		program = gl::CreateProgram();
		gl::AttachShader(program, v_shader);
		gl::AttachShader(program, f_shader);
		gl::LinkProgram(program);

		// Check for linking errors
		let mut success = gl::FALSE as GLint;
		let mut info_log = vec![0; 512];
		gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
		if success != gl::TRUE as GLint {
			gl::GetProgramInfoLog(program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
			panic!(
				"ERROR::PROGRAM::LINKING_FAILED\n{}",
				CStr::from_ptr(info_log.as_ptr() as *const i8).to_str().unwrap()
			);
		}
	}
	program
}

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

	// Compiling shaders
	let vertex_shader = compile_shader(v_shader_src, gl::VERTEX_SHADER);
	let fragment_shader = compile_shader(f_shader_src, gl::FRAGMENT_SHADER);
	let shader_program = link_program(vertex_shader, fragment_shader);

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
			gl::UseProgram(shader_program);
			gl::BindVertexArray(vao);
			gl::DrawArrays(gl::TRIANGLES, 0, 3);
		}

		window.update();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}

	// Cleaning
	unsafe {
		gl::DeleteVertexArrays(1, &vao);
		gl::DeleteProgram(shader_program);
		gl::DeleteShader(vertex_shader);
		gl::DeleteShader(fragment_shader);
	}

	Ok(())
}
