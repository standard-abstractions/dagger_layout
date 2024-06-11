use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Attributes {
	panel_size:		Vec2<DistancePercentRemainingAuto>,
	panel_padding:	Slice4<DistancePercent>,
	panel_margin:	Slice4<DistancePercent>,
	panel_color:	Color,

	corner_size:	Slice4<Vec2<DistancePercent>>,
	corner_type:	Slice4<CornerType>,

	edge_border_thickness:		Slice4<Distance>,
	edge_border_color:			Slice4<Color>,
	corner_border_thickness:	Slice4<Distance>,
	corner_border_color:		Slice4<Color>,

	layout_self:		LayoutSelf,
	layout_children:	LayoutChildren,
}

impl Default for Attributes {
	fn default() -> Self {
		Self {
			panel_size: Vec2::broadcast(DistancePercentRemainingAuto::Auto),
			panel_padding: [DistancePercent::Pixels(0);4],
			panel_margin: [DistancePercent::Pixels(0);4],
			panel_color: [0.0;4],

			corner_size: [Vec2::broadcast(DistancePercent::Pixels(0));4],
			corner_type: [CornerType::default();4],

			edge_border_thickness: [Distance::Pixels(0);4],
			edge_border_color: [[0.0;4];4],
			corner_border_thickness: [Distance::Pixels(0);4],
			corner_border_color: [[0.0;4];4],

			layout_self: LayoutSelf::default(),
			layout_children: LayoutChildren::default(),
		}
	}
}

/*
Length				Pixel | VW | VH
LengthPercent			Length | PercentOfParent
LengthPercentRemaining		LengthPercent | Remaining


panel_size			Vec2 LengthPercentRemaining
panel_color			Color
panel_padding			Slice4 LengthPercent
panel_margin			Slice4 LengthPercent

corner_size			Slice4 Vec2 LengthPercent
corner_type			Slice4 (Rectangle | Polygon(usize) | Circle)				default = Rectangle

edge_border_thickness		Slice4 Length
edge_border_color		Slice4 Color
corner_border_thickness		Slice4 Length
corner_border_color		Slice4 Color

layout_self			None | Flow(alignment) | Positioned(origin, Vec2 LengthPercent)		default = Flow(inherit)
layout_children			None | Stacked(direction, alignment, gap)					default = Stacked(row, center)

*/