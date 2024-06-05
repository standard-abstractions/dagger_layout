use super::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Size {
	pub pixels: PixelSize,
	pub parent_percentage: FloatSize,
	pub remaining_share: FloatSize,
}

impl Size {
	pub fn calculate(&self, context: SizeCalculationContext) -> FloatSize {
		((context.remaining_space / context.remaining_children.as_()) * self.remaining_share) +
		((self.parent_percentage / 100.0) * context.parent_size) +
		self.pixels.as_()
	}

	pub fn with_width_pixels(self, pixels: isize) -> Self {
		Self {
			pixels: PixelSize::new(pixels, self.pixels.y),
			parent_percentage: self.parent_percentage,
			remaining_share: self.remaining_share,
		}
	}

	pub fn with_height_pixels(self, pixels: isize) -> Self {
		Self {
			pixels: PixelSize::new(self.pixels.x, pixels),
			parent_percentage: self.parent_percentage,
			remaining_share: self.remaining_share,
		}
	}

	pub fn with_pixels(self, pixels: PixelSize) -> Self {
		Self {
			pixels,
			parent_percentage: self.parent_percentage,
			remaining_share: self.remaining_share,
		}
	}
}