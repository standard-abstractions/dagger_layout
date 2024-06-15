use super::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum Alignment {
	#[default]
	Top,
	Center,
	Bottom,
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum PositionedOrigin {
	#[default]
	Parent,
	Screen,
}

#[derive(Clone, PartialEq, Debug)]
pub enum LayoutSelf {
	None,
	Flow {
		alignment:		Option<Alignment>,
		resolve_order:	isize,
	},
	Positioned {
		position:	Vec2<DistancePercent>,
		origin:		PositionedOrigin,
	},
}
impl Default for LayoutSelf { 
	fn default() -> Self {
		Self::Flow { alignment: None, resolve_order: 0 }
	}
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum StackedDirection {
	#[default]
	Row,
	Column,
}

#[derive(Clone, PartialEq, Debug)]
pub enum LayoutChildren {
	None,
	Stacked {
		direction:	StackedDirection,
		alignment:	Alignment,
		gap:		DistancePercent,
	}
}
impl Default for LayoutChildren {
	fn default() -> Self {
		Self::Stacked { direction: StackedDirection::Row, alignment: Alignment::Top, gap: DistancePercent::Pixels(0) }
	}
}