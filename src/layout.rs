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
		is_column:	bool,
		alignment:	Alignment,
		gap:		Physical,
	}
}
impl LayoutChildren {
	pub fn none() -> Self {
		Self::None
	}
	pub fn row() -> Self {
		Self::Stacked { is_column: false, alignment: Alignment::Top, gap: 0 }
	}
	pub fn column() -> Self {
		Self::Stacked { is_column: true, alignment: Alignment::Top, gap: 0 }
	}
	pub fn with_gap(self, gap: Physical) -> Self {
		match self {
			Self::None => Self::None,
			Self::Stacked { is_column, alignment, .. } => Self::Stacked { is_column, alignment, gap }
		}
	}
}