use super::*;

pub type Style = std::collections::HashMap<String, Attributes>;

#[derive(Clone, PartialEq, Debug)]
pub struct Attributes {
	pub panel_width:		DistancePercentRemainingAuto,
	pub panel_height:		DistancePercentRemainingAuto,
	pub panel_minimum_width:	DistancePercentRemainingAuto,
	pub panel_minimum_height:	DistancePercentRemainingAuto,
	pub panel_maximum_width:	DistancePercentRemainingAuto,
	pub panel_maximum_height:	DistancePercentRemainingAuto,
	pub panel_padding:		Slice4<DistancePercent>,
	pub panel_margin:		Slice4<DistancePercent>,
	pub panel_color:		Color,

	pub corner_size:	Slice4<Vec2<DistancePercent>>,
	pub corner_type:	Slice4<CornerType>,

	pub edge_border_thickness:		Slice4<Distance>,
	pub edge_border_color:			Slice4<Color>,
	pub corner_border_thickness:	Slice4<Distance>,
	pub corner_border_color:		Slice4<Color>,

	pub layout_self:		layout::LayoutSelf,
	pub layout_children:	layout::LayoutChildren,
}

impl Attributes {
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
			panel_padding: [DistancePercent::Pixels(0), DistancePercent::Pixels(0), DistancePercent::Pixels(0), DistancePercent::Pixels(0)],
			panel_margin: [DistancePercent::Pixels(0), DistancePercent::Pixels(0), DistancePercent::Pixels(0), DistancePercent::Pixels(0)],
			panel_color: Color::default(),

			corner_size: [Vec2::new(DistancePercent::Pixels(0), DistancePercent::Pixels(0));4],
			corner_type: [CornerType::default();4],

			edge_border_thickness: [Distance::Pixels(0);4],
			edge_border_color: [Color::default();4],
			corner_border_thickness: [Distance::Pixels(0);4],
			corner_border_color: [Color::default();4],

			layout_self: layout::LayoutSelf::default(),
			layout_children: layout::LayoutChildren::default(),
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