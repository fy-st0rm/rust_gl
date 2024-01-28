use sdl2::{
	video::Window,
	render::WindowCanvas,
	pixels::Color,
	rect::Rect,
};

pub struct Renderer {
	pub canvas: WindowCanvas,
}

impl Renderer {
	pub fn new(window: Window) -> Result<Renderer, String> {
		let canvas = window
			.into_canvas()
			.build()
			.map_err(|e| e.to_string())?;
		Ok(Renderer { canvas })
	}

	pub fn clear(&mut self, color: Color) {
		self.canvas.set_draw_color(color);
		self.canvas.clear();
	}

	pub fn update(&mut self) {
		self.canvas.present();
	}

	pub fn draw_rect(&mut self, rect: Rect, color: Color) -> Result<(), String> {
		self.canvas.set_draw_color(color);
		self.canvas.fill_rect(rect)?;
		Ok(())
	}
}
