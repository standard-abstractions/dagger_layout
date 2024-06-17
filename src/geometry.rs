use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Geometry {
	pub panel_position: 	Vec2<Physical>,
	pub panel_size: 		Vec2<Physical>,
	pub panel_color:		Color,

	pub corner_size:	Slice4<Vec2<DistancePercent>>,
	pub corner_type:	Slice4<attributes::CornerType>,

	pub edge_border_thickness:		Slice4<Physical>,
	pub edge_border_color:			Slice4<Color>,
	pub corner_border_thickness:	Slice4<Physical>,
	pub corner_border_color:		Slice4<Color>,
}

impl Geometry {
	pub fn quad(&self) -> Vec<Vertex> {
		vec![
			// Triangle 1
			Vertex::new(self.panel_position, self.panel_color),
			Vertex::new(self.panel_position + (0, self.panel_size.y), self.panel_color),
			Vertex::new(self.panel_position + (self.panel_size.x, 0), self.panel_color),

			// Triangle 2
			Vertex::new(self.panel_position + (0, self.panel_size.y), self.panel_color),
			Vertex::new(self.panel_position + (self.panel_size.x, 0), self.panel_color),
			Vertex::new(self.panel_position + self.panel_size, self.panel_color),
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
			where F: ?Sized + Facade{
				let vertices: Vec<GliumVertex> = geometries
					.iter()
					.flat_map(|geometry| {
						geometry.quad()
							.iter()
							.map(|vertex| GliumVertex::from(vertex.clone()).to_screen_space(screen_size))
							.collect::<Vec<GliumVertex>>()
					})
					.collect::<Vec<GliumVertex>>();
				VertexBuffer::new(display, &vertices).unwrap()
			}
		}
	}
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vertex {
	position: 	Vec2<Physical>,
	color:		Color,
}

impl Vertex {
	pub fn new(position: Vec2<Physical>, color: Color) -> Self { Self { position, color } }
}

cfg_if::cfg_if! {
	if #[cfg(feature = "glium")] {
		#[derive(Clone, Copy, PartialEq, Debug)]
		pub struct GliumVertex {
			position: 	[Abstract;2],
			color:		[f32;4],
		}
		glium::implement_vertex!(GliumVertex, position, color);

		impl GliumVertex {
			fn to_screen_space(&self, screen_size: Vec2<Physical>) -> Self {
				let mut position = Vec2::from(self.position) / screen_size.as_() * 2.0 - 1.0;
				position.y *= -1.0;
				Self {
					position: position.into_array(),
					color: self.color,
				}
			}
		}

		impl From<Vertex> for GliumVertex {
			fn from(value: Vertex) -> Self {
				Self {
					position: value.position.as_().into_array(),
					color: value.color.to_glium(),
				}
			}
		}
	}
}
