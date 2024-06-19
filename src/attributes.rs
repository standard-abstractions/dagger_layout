use super::*;

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Style {
	pub normal: Attributes,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Attributes {
	pub panel_width:			DistancePercentRemainingAuto,
	pub panel_height:			DistancePercentRemainingAuto,
	pub panel_minimum_width:	DistancePercentRemainingAuto,
	pub panel_minimum_height:	DistancePercentRemainingAuto,
	pub panel_maximum_width:	DistancePercentRemainingAuto,
	pub panel_maximum_height:	DistancePercentRemainingAuto,
	pub panel_padding:			Slice4<DistancePercent>,
	pub panel_margin:			Slice4<DistancePercent>,
	pub panel_background:		Background,

	pub corner_size:	Slice4<Vec2<DistancePercent>>,
	pub corner_type:	Slice4<CornerType>,

	pub edge_border_thickness:		Slice4<Physical>,
	pub edge_border_color:			Slice4<Color>,
	pub corner_border_thickness:	Slice4<Physical>,
	pub corner_border_color:		Slice4<Color>,

	pub layout_self:		layout::LayoutSelf,
	pub layout_children:	layout::LayoutChildren,
}

impl Attributes {
	pub fn panel_width(self, panel_width: DistancePercentRemainingAuto) -> Self { Self { panel_width, ..self } }
	pub fn panel_height(self, panel_height: DistancePercentRemainingAuto) -> Self { Self { panel_height, ..self } }
	pub fn panel_padding(self, panel_padding: Slice4<DistancePercent>) -> Self { Self { panel_padding, ..self } }
	pub fn panel_background(self, panel_background: Background) -> Self { Self { panel_background, ..self } }
	pub fn layout_children(self, layout_children: layout::LayoutChildren) -> Self { Self { layout_children, ..self } }

	pub fn in_flow(&self) -> bool {
		match self.layout_self {
			layout::LayoutSelf::Flow { .. } => true,
			_ => false,
		}
	}
}

impl Default for Attributes {
	fn default() -> Self {
		Self {
			panel_width: DistancePercentRemainingAuto::Auto,
			panel_height: DistancePercentRemainingAuto::Auto,
			panel_minimum_width: DistancePercentRemainingAuto::Pixels(0),
			panel_minimum_height: DistancePercentRemainingAuto::Pixels(0),
			panel_maximum_width: DistancePercentRemainingAuto::Pixels(Physical::MAX),
			panel_maximum_height: DistancePercentRemainingAuto::Pixels(Physical::MAX),
			panel_padding: [DistancePercent::Pixels(0);4],
			panel_margin: [DistancePercent::Pixels(0);4],
			panel_background: Background::Color(Color::null()),

			corner_size: [Vec2::new(DistancePercent::Pixels(0), DistancePercent::Pixels(0));4],
			corner_type: [CornerType::default();4],

			edge_border_thickness: [0;4],
			edge_border_color: [Color::black();4],
			corner_border_thickness: [0;4],
			corner_border_color: [Color::black();4],

			layout_self: layout::LayoutSelf::default(),
			layout_children: layout::LayoutChildren::row(),
		}
	}
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum CornerType {
	#[default]
	Rectangle,
	Polygon(usize),
	Circle,
}