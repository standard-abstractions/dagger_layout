use super::*;

/* Structure */
pub type Slice4<T> = [T;4];

/* Size */
pub type Physical = isize;
pub type Abstract = f32;

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
	Calculate(Vec<Self>),
}

/* Style */
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Color(Vec4<u8>);
cfg_if::cfg_if! {
	if #[cfg(feature = "glium")] {
		impl Color { pub fn to_glium(&self) -> [f32;4] { self.0.as_().into_array() } }
	}
}
