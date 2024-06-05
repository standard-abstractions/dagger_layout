use super::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct LayoutTree<T> {
	marker: std::marker::PhantomData<T>,
}

