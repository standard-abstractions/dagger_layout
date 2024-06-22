use super::*;

/* Structure */
pub type Slice4<T> = [T;4];

/* Size */
pub type Physical = isize;
pub type Abstract = f32;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DistancePercent {
	Pixels(Physical),
	Percent(Abstract),
}

impl DistancePercent {
	pub fn calculate(&self, parent_size: Physical) -> Physical {
		match self {
			Self::Pixels(pixels) => *pixels,
			Self::Percent(percent) => (*percent * (parent_size as Abstract / 100.0)) as Physical,
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

	pub fn calculate(&self, context: SizeCalculationContext, is_height: bool) -> Physical {
		if is_height {
			match self {
				Self::Pixels(pixels) => *pixels,
				Self::Percent(percent) => (*percent * (context.parent_size.y as Abstract / 100.0)) as Physical,
				Self::Remaining(remaining) => (*remaining * (context.remaining_space.y / context.remaining_children.y) as Abstract) as Physical,
				Self::Auto => 0,
			}
		} else {
			match self {
				Self::Pixels(pixels) => *pixels,
				Self::Percent(percent) => (*percent * (context.parent_size.x as Abstract / 100.0)) as Physical,
				Self::Remaining(remaining) => (*remaining * (context.remaining_space.x / context.remaining_children.x) as Abstract) as Physical,
				Self::Auto => 0,
			}
		}
	}
}

#[derive(Clone, PartialEq, Debug)]
pub enum Scalar<T> {
	Single(T),
	Calculate(Vec<T>),
}

/* Style */
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color(Vec4<u8>);
impl Color {
	pub fn rgba(rgba: Vec4<u8>) -> Self { Self(rgba) }
	pub fn rgb(rgb: Vec3<u8>) -> Self { Self(Vec4::new(rgb.x, rgb.y, rgb.z, 255)) }
	pub fn null() -> Self { Self(Vec4::zero()) }
	pub fn black() -> Self { Self(Vec4::new(0, 0, 0, 255)) }
	pub fn white() -> Self { Self(Vec4::new(255, 255, 255, 255)) }
	pub fn red() -> Self { Self(Vec4::new(255, 0, 0, 255)) }
	pub fn green() -> Self { Self(Vec4::new(0, 255, 0, 255)) }
	pub fn blue() -> Self { Self(Vec4::new(0, 0, 255, 255)) }
}
cfg_if::cfg_if! {
	if #[cfg(feature = "glium")] {
		impl Color { pub fn to_glium(&self) -> [f32;4] { (self.0.as_() / 255.0).into_array() } }
	}
}