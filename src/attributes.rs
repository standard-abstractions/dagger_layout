use std::collections::HashMap;

use super::*;

type FourSlice<T> = [T;4]; // Start with top (or top-right), go clockwise
type NineSlice<T> = [T;9]; // ''

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Reservable<T> {
	Overlay(T),
	Reserved(T),
}

#[derive(Clone, PartialEq, Debug)]
pub struct PanelAttributes {
	preferred_size:			Vec2<Size>,
	minimum_size:			Vec2<Size>,
	maximum_size:			Vec2<Size>,
	padding:				Reservable<FourSlice<Size>>,
	margin:					Reservable<FourSlice<Size>>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BorderStroke { Inner, Outer, Half }

#[derive(Clone, PartialEq, Debug)]
pub struct BorderAttributes {
	thickness: NineSlice<Size>,
	include_border_in_size: NineSlice<BorderStroke>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Alignment { Top, Center, Bottom }

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PositionOrigin { Parent, Screen }

#[derive(Clone, PartialEq, Debug)]
pub enum SelfScheme {
	None,
	Flow {
		align_self:	Option<Alignment>,
	},
	Positioned {
		position:	Vec2<Size>,
		origin:		PositionOrigin,
	},
}

#[derive(Clone, PartialEq, Debug)]
pub enum ChildrenScheme {
	None,
	Stacked {
		direction:		bool,
		align_children:	Alignment,
		gap:			Size,
	},
}

#[derive(Clone, PartialEq, Debug)]
pub struct Attributes {
	panel:					PanelAttributes,
	border: 				BorderAttributes,

	layout_self:			SelfScheme,
	layout_children:		ChildrenScheme,
}

#[derive(Clone, PartialEq, Debug)]
pub enum State {
	Normal,
	Hovered,
	Focused,
	Custom(String),
}

pub type Style = HashMap<State, Attributes>;
