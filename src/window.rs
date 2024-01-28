use sdl2::{
	Sdl,
	event::EventPollIterator,
	video::{
		GLProfile,
		GLContext,
	},
};
use gl;

pub struct Window {
	pub sdl_context: Sdl,
	pub gl_context: GLContext,
	pub sdl_window: sdl2::video::Window,
	pub event_pump: sdl2::EventPump,
}

impl Window {
	pub fn new(title: &str, width: i32, height: i32) -> Result<Window, String> {
		let sdl_context = sdl2::init()?;
		let video_subsystem = sdl_context.video()?;

		let gl_attr = video_subsystem.gl_attr();
		gl_attr.set_context_profile(GLProfile::Core);
		gl_attr.set_context_version(3, 3);

		let window = video_subsystem.window(title, width as u32, height as u32)
			.opengl()
			.position_centered()
			.build()
			.map_err(|err| err.to_string())?;
	
		let ctx = window.gl_create_context()?;
		gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

		let event_pump = sdl_context.event_pump()?;

		Ok(Window {
			sdl_context: sdl_context,
			gl_context: ctx,
			sdl_window: window,
			event_pump: event_pump
		})
	}

	pub fn get_events(&mut self) -> EventPollIterator {
		self.event_pump.poll_iter()
	}

	pub fn update(&mut self) {
		self.sdl_window.gl_swap_window();
	}

	pub fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
		unsafe {
			gl::ClearColor(r, g, b, a);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}
	}
}
