use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Geometry {
	pub panel_position: 	Vec2<Physical>,
	pub panel_size: 		Vec2<Physical>,
	pub panel_color:		Color,
	pub panel_background:	Option<u32>,

	pub corner_size:	Slice4<Vec2<DistancePercent>>,
	pub corner_type:	Slice4<attributes::CornerType>,

	pub edge_border_thickness:		Slice4<Physical>,
	pub edge_border_color:			Slice4<Color>,
	pub corner_border_thickness:	Slice4<Physical>,
	pub corner_border_color:		Slice4<Color>,
}

impl Geometry {
	pub fn quad(&self) -> [Vertex;6] {
		[
			// Triangle 1
			Vertex::new(self.panel_position, self.panel_color, self.panel_background, [0.0, 0.0]),
			Vertex::new(self.panel_position + (0, self.panel_size.y), self.panel_color, self.panel_background, [0.0, 1.0]),
			Vertex::new(self.panel_position + self.panel_size, self.panel_color, self.panel_background, [1.0, 1.0]),
			
			// Triangle 2
			Vertex::new(self.panel_position, self.panel_color, self.panel_background, [0.0, 0.0]),
			Vertex::new(self.panel_position + (self.panel_size.x, 0), self.panel_color, self.panel_background, [1.0, 0.0]),
			Vertex::new(self.panel_position + self.panel_size, self.panel_color, self.panel_background, [1.0, 1.0]),
		]
	}
}

cfg_if::cfg_if! {
	if #[cfg(feature = "glium")] {
		use glium::{
			backend::Facade,
			VertexBuffer,
		};

		impl Geometry {
			pub fn create_vb_simple_quads<F>(display: &F, screen_size: Vec2<Physical>, geometries: &Vec<Geometry>) -> VertexBuffer<GliumVertex>
			where F: ?Sized + Facade {
				let mut vertices: Vec<GliumVertex> = Vec::with_capacity(geometries.len() * 6);

				for geometry in geometries {
					for vertex in geometry.quad().iter() {
						vertices.push(GliumVertex::from(*vertex).to_screen_space(screen_size));
					}
				}

				VertexBuffer::new(display, &vertices).unwrap()
			}
		}
	}
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vertex {
	position: 	Vec2<Physical>,
	color:		Color,
	background_texture_id:	Option<u32>,
	background_texture_uv:	[Abstract;2],
}

impl Vertex {
	pub fn new(position: Vec2<Physical>, color: Color, background_texture_id: Option<u32>, background_texture_uv: [Abstract;2]) -> Self { Self { position, color, background_texture_id, background_texture_uv } }
}

cfg_if::cfg_if! {
	if #[cfg(feature = "glium")] {
		#[derive(Clone, Copy, PartialEq, Debug)]
		pub struct GliumVertex {
			position: 		[Abstract;2],
			texture_uvs: 	[Abstract;2],
			texture_id:		[u32;2],
			color:			[f32;4],
		}
		glium::implement_vertex!(GliumVertex, position, color, texture_id, texture_uvs);

		impl GliumVertex {
			fn to_screen_space(&self, screen_size: Vec2<Physical>) -> Self {
				let mut position = Vec2::from(self.position) / screen_size.as_() * 2.0 - 1.0;
				position.y *= -1.0;
				Self {
					position: position.into_array(),
					color: self.color,
					texture_id: self.texture_id,
					texture_uvs: self.texture_uvs,
				}
			}
		}

		impl From<Vertex> for GliumVertex {
			fn from(value: Vertex) -> Self {
				Self {
					position: value.position.as_().into_array(),
					color: value.color.to_glium(),
					texture_id: [
						value.background_texture_id.is_some() as u32,
						value.background_texture_id.unwrap_or(0),
					],
					texture_uvs: value.background_texture_uv,
				}
			}
		}
	}
}

