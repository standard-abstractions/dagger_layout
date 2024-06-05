use super::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Vertex {
	position: Vec2<Physical>,
}

impl Vertex {
	pub fn from_position(position: Vec2<Physical>) -> Self { Self { position } }
	pub fn from_x_and_y(x: Physical, y: Physical) -> Self { Self { position: Vec2::new(x, y) } }

	pub fn to_screen_space(&self, screen_size: Vec2<Physical>) -> Self {
		let mut position = self.position / screen_size * 2 - 1;
		position.y *= -1;
		Self { position }
	}
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Quad {
	vertices: [Vertex;4],
}

impl Quad {
	pub fn from_vertices(vertices: [Vertex;4]) -> Self { Self { vertices } }
	pub fn from_position_and_size(position: Vec2<Physical>, size: Vec2<Physical>) -> Self {
		Self {
			vertices: [
				Vertex::from_position(position),
				Vertex::from_x_and_y(position.x + size.x, position.y),
				Vertex::from_x_and_y(position.x, position.y + size.y),
				Vertex::from_position(position + size),
			],
		}
	}

	pub fn to_screen_space(&self, screen_size: Vec2<Physical>) -> Self {
		Self {
			vertices: self.vertices.iter().map(|v| v.to_screen_space(screen_size)).collect::<Vec<Vertex>>().try_into().unwrap(),
		}
	}
}

cfg_if::cfg_if! {
	if #[cfg(feature = "glium")] {
		glium::implement_vertex!(Vertex, position);

		impl Quad {
			pub fn create_vertex_buffer<F>(display: &F, screen_size: Vec2<Physical>, quads: &Vec<Quad>) -> glium::VertexBuffer<Vertex>
			where F: ?Sized + glium::Facade {
				let vertices: Vec<Vertex> = quads.iter().flat_map(|quad| {
					vec![
						quad.vertices[0],
						quad.vertices[1],
						quad.vertices[2],
						quad.vertices[1],
						quad.vertices[2],
						quad.vertices[3],
					]
				}).collect();
				glium::VertexBuffer::new(display, &vertices).unwrap()
			}
		}
	}
}