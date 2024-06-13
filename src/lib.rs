pub mod attributes;
pub mod geometry;
pub mod layout;
pub mod types;

use types::*;
use vek::{Vec2, Vec4};

#[derive(Clone, PartialEq, Debug)]
pub struct Node<T> {
	pub data:			T,
	pub children:		Vec<Node<T>>,
}

impl<T> Node<T> {
	pub fn new(data: T) -> Self {
		Self { data, children: vec![] }
	}
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Context {
	parent_size:		Vec2<Physical>,
	remaining_space:	Vec2<Physical>,
	remaining_children:	Vec2<usize>,
}

pub fn calculate_size

#[derive(Clone, PartialEq, Debug)]
pub struct Element {
	style: attributes::Style,
}

pub fn calculate_size(node: &Node<Element>, context: Context) -> Node<Vec2<Physical>> {
	let style = 
	match .layout_children {
		layout::LayoutChildren::None => Node::new(Vec2::zero()),
		layout::LayoutChildren::Stacked { direction, alignment, gap } => {
			let size = style.panel_size;
			if let Some(min_size) = style.panel_min_size {
				size = Vec2::max(size, min_size) // calc all but auto
			}
		},
	}
}