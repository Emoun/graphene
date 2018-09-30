
use std::iter::FromIterator;

///
/// Trait alias
///
pub trait Id: Copy + Eq{}
impl<T> Id for T
	where T: Copy + Eq
{}

pub trait IntoFromIter<I>: IntoIterator<Item=I> + FromIterator<I>
	where
		//I: Id,

{}
impl<T, I> IntoFromIter<I> for T
	where
		T: IntoIterator<Item=I> + FromIterator<I>,
		//I: Id,
{}