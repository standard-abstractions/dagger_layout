pub mod attributes;
pub mod geometry;
pub mod layout;
pub mod types;

use itertools::Itertools;
use types::*;

use vek::{Clamp, Vec2, Vec4};

#[derive(Clone, PartialEq, Debug)]
pub struct Element {
	styles: attributes::Style,
}

impl Element {
	pub fn get_style(&self) -> attributes::Attributes {
		self.styles.get("normal").cloned().unwrap_or(attributes::Attributes::default())
	}
}

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

impl Node<Element> {
	pub fn calculate_sizes(&self, context: Context) -> Node<Vec2<Physical>> {
		let style = self.data.get_style();
	
		let ideal_width_offset = style.panel_width.calculate(context, false);
		let ideal_height_offset = style.panel_height.calculate(context, true);
		let minimum_width_offset = style.panel_minimum_width.calculate(context, false);
		let minimum_height_offset = style.panel_minimum_height.calculate(context, true);
		let maximum_width_offset = style.panel_maximum_width.calculate(context, false);
		let maximum_height_offset = style.panel_maximum_height.calculate(context, true);
	
		let mut children = vec![];
		let children_count = self.children.len();
		
		let auto_size: Vec2<Physical> = match style.layout_children {
			layout::LayoutChildren::None => Vec2::zero(),
			layout::LayoutChildren::Stacked { direction, .. } => {
				let mut total_size = Vec2::zero();

				for (index, child) in self.children.iter().enumerate() {
					let child_size = child.calculate_sizes(Context {
						parent_size: Vec2::new(ideal_width_offset, ideal_height_offset),
						remaining_space: match direction {
							layout::StackedDirection::Row => Vec2::new(ideal_width_offset, ideal_height_offset) - (total_size.x, 0),
							layout::StackedDirection::Column => Vec2::new(ideal_width_offset, ideal_height_offset) - (0, total_size.y),
						},
						remaining_children: match direction {
							layout::StackedDirection::Row => Vec2::new((children_count - index) as isize, 1),
							layout::StackedDirection::Column => Vec2::new(1, (children_count - index) as isize),
						},
					});
					total_size += child_size.data;
					children.push(child_size);
				}
	
				total_size
			},
		};
	
		let ideal_width = ideal_width_offset + if style.panel_width.is_auto() { auto_size.x } else { 0 };
		let ideal_height = ideal_height_offset + if style.panel_height.is_auto() { auto_size.y } else { 0 };
		let minimum_width = minimum_width_offset + if style.panel_minimum_width.is_auto() { auto_size.x } else { 0 };
		let minimum_height = minimum_height_offset + if style.panel_minimum_height.is_auto() { auto_size.y } else { 0 };
		let maximum_width = maximum_width_offset + if style.panel_maximum_width.is_auto() { auto_size.x } else { 0 };
		let maximum_height = maximum_height_offset + if style.panel_maximum_height.is_auto() { auto_size.y } else { 0 };
	
		Node::new(Vec2::clamp(
			Vec2::new(ideal_width, ideal_height),
			Vec2::new(minimum_width, minimum_height),
			Vec2::new(maximum_width, maximum_height),
		)).with_children(children)
	}
}