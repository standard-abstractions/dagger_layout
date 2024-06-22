pub mod attributes;
pub mod geometry;
pub mod layout;
pub mod types;

use types::*;

use vek::{Clamp, Vec2, Vec3, Vec4};

#[derive(Clone, PartialEq, Debug)]
pub struct Element {
	pub styles: attributes::Style,
}

impl Element {
	pub fn new() -> Self {
		Self { styles: attributes::Style::default() }
	}

	pub fn with_normal(self, attributes: attributes::Attributes) -> Self {
		Self { styles: attributes::Style { normal: attributes, ..self.styles } }
	}
	
	pub fn get_style(&self) -> &attributes::Attributes {
		&self.styles.normal
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

	pub fn to_vec(self) -> Vec<T> {
		let mut vec = vec![];
		vec.push(self.data);
		for child in self.children {
			vec.extend(child.to_vec());
		}
		vec
	}
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SizeCalculationContext {
	parent_size:		Vec2<Physical>,
	remaining_space:	Vec2<Physical>,
	remaining_children:	Vec2<isize>,
}

impl SizeCalculationContext {
	pub fn begin_calculation(screen_size: Vec2<Physical>) -> Self {
		Self {
			parent_size: screen_size,
			remaining_space: screen_size,
			remaining_children: Vec2::one(),
		}
	}
}

impl Node<Element> {
	pub fn calculate_sizes(&self, context: SizeCalculationContext) -> Node<Vec2<Physical>> {
		let style = self.data.get_style();
		let horizontal_padding = style.panel_padding[1].calculate(context.parent_size.x) + style.panel_padding[3].calculate(context.parent_size.x);
		let vertical_padding = style.panel_padding[0].calculate(context.parent_size.y) + style.panel_padding[2].calculate(context.parent_size.y);
		let padding = Vec2::new(horizontal_padding, vertical_padding);
	
		let minimum_width_offset = style.panel_minimum_width.calculate(context, false);
		let minimum_height_offset = style.panel_minimum_height.calculate(context, true);
		let maximum_width_offset = style.panel_maximum_width.calculate(context, false);
		let maximum_height_offset = style.panel_maximum_height.calculate(context, true);
		let ideal_width_offset = (style.panel_width.calculate(context, false)).clamp(minimum_width_offset, maximum_width_offset);
		let ideal_height_offset = (style.panel_height.calculate(context, true)).clamp(minimum_height_offset, maximum_height_offset);
	
		let children_count = self.children.len();
		let mut children = Vec::with_capacity(children_count);
		
		let auto_size: Vec2<Physical> = match style.layout_children {
			layout::LayoutChildren::None => Vec2::zero(),
			layout::LayoutChildren::Stacked { is_column, gap, .. } => {
				let mut total_size = Vec2::zero();

				let parent_size = Vec2::new(ideal_width_offset, ideal_height_offset) - padding;
				for (index, child) in self.children.iter().enumerate() {
					let remaining_children = if is_column {
						Vec2::new(1, (children_count - index) as isize)
					} else {
						Vec2::new((children_count - index) as isize, 1)
					};

					let remaining_space = parent_size - if is_column {
						(0, (gap * (remaining_children.y - 1)) + total_size.y)
					} else {
						((gap * (remaining_children.x - 1)) + total_size.x, 0)
					};

					let child_size = child.calculate_sizes(SizeCalculationContext {
						parent_size,
						remaining_space,
						remaining_children,
					});

					if child.data.get_style().in_flow() {
						total_size += child_size.data;
						if index < children_count - 1 {
							total_size += gap;
						}
					}

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
	
		let size = Vec2::clamp(
			Vec2::new(ideal_width, ideal_height),
			Vec2::new(minimum_width, minimum_height),
			Vec2::new(maximum_width, maximum_height),
		);
		Node::new(size).with_children(children)
	}

	pub fn calculate_positions(&self, sizes: &Node<Vec2<Physical>>, current_position: Vec2<Physical>) -> Node<Vec2<Physical>> {
		let style = self.data.get_style();

		let children_count = self.children.len();
		let mut children = Vec::with_capacity(children_count);

		match style.layout_children {
			layout::LayoutChildren::None => {},
			layout::LayoutChildren::Stacked { is_column, gap, .. } => {
				let mut next_position = Vec2::new(style.panel_padding[1].calculate(sizes.data.x), style.panel_padding[0].calculate(sizes.data.y));
				for (index, (child, size)) in self.children.iter().zip(sizes.children.iter()).enumerate() {
					children.push(child.calculate_positions(size, current_position + next_position));
					
					if is_column {
						next_position += (0, size.data.y);
						if index < children_count {
							next_position.y += gap;
						}
					} else {
						next_position += (size.data.x, 0);
						if index < children_count {
							next_position.x += gap;
						}
					}

				}
			},
		}

		Node::new(current_position).with_children(children)
	}

	fn impl_calculate_geometries(&self, sizes: &Node<Vec2<Physical>>, positions: &Node<Vec2<Physical>>) -> Node<geometry::Geometry> {
		let style = self.data.get_style();

		let children = self.children.iter()
			.zip(sizes.children.iter().zip(positions.children.iter()))
			.map(|(child, (size, position))| child.impl_calculate_geometries(size, position))
			.collect();

		let geometry = geometry::Geometry {
			panel_position: positions.data,
			panel_size: sizes.data,
			panel_color: style.panel_color,
			panel_background: style.panel_background,
			corner_size: style.corner_size,
			corner_type: style.corner_type,
			edge_border_thickness: style.edge_border_thickness,
			edge_border_color: style.edge_border_color,
			corner_border_thickness: style.corner_border_thickness,
			corner_border_color: style.corner_border_color,
		};

		Node::new(geometry).with_children(children)
	}

	pub fn calculate_geometries(&self, context: SizeCalculationContext) -> Node<geometry::Geometry> {
		let sizes = self.calculate_sizes(context);
		let positions = self.calculate_positions(&sizes, Vec2::zero());
		self.impl_calculate_geometries(&sizes, &positions)
	}
}