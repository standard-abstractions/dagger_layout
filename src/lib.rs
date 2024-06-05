pub mod attributes;
pub mod geometry;
pub mod layout;

use vek::Vec2;

type Physical = isize;
type Abstract = f32;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Dimension {
	Pixels(Physical),
	Percent(Abstract),
	Remaining(Abstract),
}

#[derive(Clone, PartialEq, Default, Debug)]
pub enum Size {
	Single(Dimension),
	Many(Vec<Dimension>),
	#[default]
	Auto,
	AutoWithMargin(Vec<Dimension>),
	AutoWithAspectRatio(Abstract), //Percentage of other axis
}