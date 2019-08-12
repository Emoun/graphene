
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

pub trait EdgeIntoFromIter<'a, V, W:'a>: IntoFromIter<(V, V, &'a W)>
{}
impl<'a,T,V,W:'a> EdgeIntoFromIter<'a, V, W> for T
	where
		T: IntoFromIter<(V, V, &'a W)>,
{}

pub trait EdgeIntoFromIterMut<'a, V, W:'a>: IntoFromIter<(V, V, &'a mut W)>
{}
impl<'a,T,V,W:'a> EdgeIntoFromIterMut<'a, V, W> for T
	where
		T: IntoFromIter<(V, V, &'a mut W)>,
{}


