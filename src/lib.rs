pub mod attributes;
pub mod geometry;
pub mod layout;

use vek::Vec2;
type Slice4<T> = [T;4];

type Physical = isize;
type Abstract = f32;

type Color = [f32;4];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Distance {
	Pixels(Physical),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DistancePercent {
	Pixels(Physical),
	Percent(Abstract),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DistancePercentRemaining {
	Pixels(Physical),
	Percent(Abstract),
	Remaining(Abstract),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DistancePercentRemainingAuto {
	Pixels(Physical),
	Percent(Abstract),
	Remaining(Abstract),
	Auto,
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum CornerType {
	#[default]
	Rectangle,
	Polygon(usize),
	Circle,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LayoutSelf {
	None,
	Flow {
		alignment:	Option<Alignment>,
	},
	Positioned {
		position:	Vec2<DistancePercent>,
		origin:		PositionedOrigin
	},
}
impl Default for LayoutSelf { fn default() -> Self { Self::Flow { alignment: None } } }

#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum StackedDirection {
	#[default]
	Row,
	Column,
}

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