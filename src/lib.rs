pub mod attributes;
pub mod geometry;
pub mod layout;
pub mod types;

use itertools::Itertools;
use types::*;

use vek::{Clamp, Vec2, Vec4};

#[derive(Clone, PartialEq, Debug)]
pub struct Node<T> {
	pub data:			T,
	pub children:		Vec<Node<T>>,
}

impl<T> Node<T> {
	pub fn new(data: T) -> Self {
		Self { data, children: vec![] }
	}

	pub fn with_children(self, children: Vec<Node<T>>) -> Self {
		Self { data: self.data, children }
	}
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Context {
	parent_size:		Vec2<Physical>,
	remaining_space:	Vec2<Physical>,
	remaining_children:	Vec2<isize>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Element {
	styles: attributes::Style,
}

impl Element {
	pub fn get_style(&self) -> attributes::Attributes {
		self.styles.get("normal").cloned().unwrap_or(attributes::Attributes::default())
	}
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Quad {
	position:		Vec2<Physical>,
	size:			Vec2<Physical>,
}

pub fn calculate_size(node: &Node<Element>, context: Context) -> Node<Quad> {
	let style = node.data.get_style();

	let ideal_width_offset = style.panel_width.non_auto_part().calculate(context.parent_size.x, context.remaining_space.x, context.remaining_children.x);
	let ideal_height_offset = style.panel_height.non_auto_part().calculate(context.parent_size.y, context.remaining_space.y, context.remaining_children.y);
	let minimum_width_offset = style.panel_minimum_width.non_auto_part().calculate(context.parent_size.x, context.remaining_space.x, context.remaining_children.x);
	let minimum_height_offset = style.panel_minimum_height.non_auto_part().calculate(context.parent_size.y, context.remaining_space.y, context.remaining_children.y);
	let maximum_width_offset = style.panel_maximum_width.non_auto_part().calculate(context.parent_size.x, context.remaining_space.x, context.remaining_children.x);
	let maximum_height_offset = style.panel_maximum_height.non_auto_part().calculate(context.parent_size.y, context.remaining_space.y, context.remaining_children.y);

	let children: Vec<Node<Quad>> = vec![];
	let auto_size: Vec2<Physical> = match style.layout_children {
		layout::LayoutChildren::None => Vec2::zero(),
		layout::LayoutChildren::Stacked { direction, alignment, gap } => {
			todo!();

			Vec2::zero()
		},
	};

	let ideal_width = ideal_width_offset + if style.panel_width.is_auto() { auto_size.x } else { 0 };
	let ideal_height = ideal_height_offset + if style.panel_height.is_auto() { auto_size.y } else { 0 };
	let minimum_width = minimum_width_offset + if style.panel_minimum_width.is_auto() { auto_size.x } else { 0 };
	let minimum_height = minimum_height_offset + if style.panel_minimum_height.is_auto() { auto_size.y } else { 0 };
	let maximum_width = maximum_width_offset + if style.panel_maximum_width.is_auto() { auto_size.x } else { 0 };
	let maximum_height = maximum_height_offset + if style.panel_maximum_height.is_auto() { auto_size.y } else { 0 };

	let size = Vec2::clamp(
		Vec2::new(ideal_width, ideal_height),
		Vec2::new(minimum_width, minimum_height),
		Vec2::new(maximum_width, maximum_height),
	);

	Node::new(Quad {
		position: Vec2::zero(),
		size,
	}).with_children(children)
}