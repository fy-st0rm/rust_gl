use crate::math::vec::*;
use crate::shader::Shader;
use crate::static_assert;

use gl;
use gl::types::*;
use std::ptr;
use std::mem::{ size_of, offset_of };


const V_DEF_SHADER_SRC: &str = r#"
	#version 440 core
	layout (location = 0) in vec3 pos;
	void main() {
		gl_Position = vec4(pos, 1.0);
	}
"#;

const F_DEF_SHADER_SRC: &str = r#"
	#version 440 core
	layout (location = 0) out vec4 color;
	void main() {
		color = vec4(1, 0, 0, 1.0);
	}
"#;


pub struct Vertex {
	pub pos: Vec3,
}

pub struct Renderer {
	vao: GLuint,
	vbo: GLuint,
	pub buffer: Vec<Vertex>,
	buff_idx: i32,
	shader: Shader,
}

// Renderer configs
const TEXTURE_SAMPLE_AMT: i32 =  32;
const VERTEX_SIZE : i32       = 3;
const MAX_VERT_CNT: i32       = 10000;
const MAX_BUFF_CAP: i32       = MAX_VERT_CNT  * VERTEX_SIZE;
const MAX_VBO_SIZE: usize     = MAX_BUFF_CAP as usize * size_of::<f32>();

static_assert!(
	VERTEX_SIZE as usize == size_of::<Vertex>() / size_of::<f32>(),
	"Size of vertex missmatched"
);

impl Renderer {
	pub fn new() -> Result<Renderer, String> {
		let mut vao = 0;
		let mut vbo = 0;
		let buffer: Vec<Vertex> = Vec::new();
		let shader = Shader::from_src(V_DEF_SHADER_SRC, F_DEF_SHADER_SRC)?;

		unsafe {
			gl::Enable(gl::DEPTH_TEST);

			// VAO
			gl::GenVertexArrays(1, &mut vao);
			gl::BindVertexArray(vao);

			// VBO
			gl::GenBuffers(1, &mut vbo);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::BufferData(
				gl::ARRAY_BUFFER,
				MAX_VBO_SIZE as GLsizeiptr,
				ptr::null(),
				gl::DYNAMIC_DRAW
			);

			// Setting up vertex format
			gl::VertexAttribPointer(
				0, 3, gl::FLOAT, gl::FALSE,
				3 * size_of::<GLfloat>() as GLsizei,
				offset_of!(Vertex, pos) as *const GLvoid
			);
			gl::EnableVertexAttribArray(0);

			// Unbinding
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			gl::BindVertexArray(0);
		}

		Ok(Renderer {
			vao: vao,
			vbo: vbo,
			buffer: buffer,
			buff_idx: 0,
			shader: shader,
		})
	}

	pub fn delete(&self) {
		self.shader.delete();
		unsafe {
			gl::DeleteVertexArrays(1, &self.vao);
		}
	}

	pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
		unsafe {
			gl::ClearColor(r, g, b, a);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		}
	}

	pub fn begin(&mut self) {
		self.shader.bind();
		self.buff_idx = 0;
		self.buffer.clear();
	}

	pub fn end(&self) {
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
			gl::BufferSubData(
				gl::ARRAY_BUFFER,
				0,
				(self.buffer.len() * size_of::<Vertex>()) as GLsizeiptr,
				self.buffer.as_ptr() as *const GLvoid,
			);

			gl::BindVertexArray(self.vao);
			gl::DrawArrays(gl::TRIANGLES, 0, self.buff_idx);
		}
	}

	pub fn push_vertex(&mut self, v: Vertex) {
		self.buffer.push(v);
		self.buff_idx += VERTEX_SIZE;
	}
}
