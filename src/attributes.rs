use super::*;

pub type Style = std::collections::HashMap<String, Attributes>;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Attributes {
	pub panel_size:		Vec2<DistancePercentRemainingAuto>,
	pub panel_min_size:	Vec2<Option<DistancePercentRemainingAuto>>,
	pub panel_max_size:	Vec2<Option<DistancePercentRemainingAuto>>,
	pub panel_padding:	Slice4<DistancePercent>,
	pub panel_margin:	Slice4<DistancePercent>,
	pub panel_color:	Color,

	pub corner_size:	Slice4<Vec2<DistancePercent>>,
	pub corner_type:	Slice4<CornerType>,

	pub edge_border_thickness:		Slice4<Distance>,
	pub edge_border_color:			Slice4<Color>,
	pub corner_border_thickness:	Slice4<Distance>,
	pub corner_border_color:		Slice4<Color>,

	pub layout_self:		layout::LayoutSelf,
	pub layout_children:	layout::LayoutChildren,
}

impl Default for Attributes {
	fn default() -> Self {
		Self {
			panel_size: Vec2::broadcast(DistancePercentRemainingAuto::Auto),
			panel_min_size: Vec2::broadcast(None),
			panel_max_size: Vec2::broadcast(None),
			panel_padding: [DistancePercent::Pixels(0);4],
			panel_margin: [DistancePercent::Pixels(0);4],
			panel_color: Color::default(),

			corner_size: [Vec2::broadcast(DistancePercent::Pixels(0));4],
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