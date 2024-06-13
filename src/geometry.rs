use super::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Vertex {
	position: 	Vec2<Physical>,
	color:		Color,
}

impl Vertex {
	pub fn new(position: Vec2<Physical>, color: Color) -> Self { Self { position, color } }

	pub fn to_screen_space(&self, screen_size: Vec2<Physical>) -> Self {
		let mut position = self.position / screen_size * 2 - 1;
		position.y *= -1;
		Self { position, color: self.color }
	}
}

cfg_if::cfg_if! {
	if #[cfg(feature = "glium")] {
		glium::implement_vertex!(Vertex, position, color);
	}
}