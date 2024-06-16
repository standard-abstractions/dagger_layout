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

impl DistancePercentRemaining {
	pub fn calculate(&self, parent_size: Physical, remaining_space: Physical, remaining_children: isize) -> Physical {
		match self {
			Self::Pixels(pixels) => *pixels,
			Self::Percent(percent) => (*percent * parent_size as Abstract) as Physical,
			Self::Remaining(remaining) => (*remaining * (remaining_space / remaining_children) as Abstract) as Physical,
		}
	}
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DistancePercentRemainingAuto {
	Pixels(Physical),
	Percent(Abstract),
	Remaining(Abstract),
	Auto,
}

impl DistancePercentRemainingAuto {
	pub fn is_auto(&self) -> bool {
		match self {
			Self::Auto => true,
			_ => false,
		}
	}

	pub fn non_auto_part(&self) -> DistancePercentRemaining {
		match self {
			Self::Pixels(pixels) => DistancePercentRemaining::Pixels(*pixels),
			Self::Percent(percent) => DistancePercentRemaining::Percent(*percent),
			Self::Remaining(remaining) => DistancePercentRemaining::Remaining(*remaining),
			Self::Auto => panic!("Called non_auto_part on an Auto!"),
		}
	}

	pub fn calculate(&self, context: Context, is_height: bool) -> Physical {
		if is_height {
			self.non_auto_part().calculate(context.parent_size.y, context.remaining_space.y, context.remaining_children.y)
		} else {
			self.non_auto_part().calculate(context.parent_size.x, context.remaining_space.x, context.remaining_children.x)
		}
	}
}

#[derive(Clone, PartialEq, Debug)]
pub enum Scalar<T> {
	Single(T),
	Calculate(Vec<T>),
}

/* Style */
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Color(Vec4<u8>);
cfg_if::cfg_if! {
	if #[cfg(feature = "glium")] {
		impl Color { pub fn to_glium(&self) -> [f32;4] { self.0.as_().into_array() } }
	}
}
