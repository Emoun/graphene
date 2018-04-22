
use ::core::Id;

pub trait Edge<V,E>
	where V: Id
{
	fn source(&self) -> &V;
	fn sink(&self) -> &V;
	fn edge(&self) -> &E;
}

impl<V,E> Edge<V,E> for (V,V,E)
	where V:Id
{
	fn source(&self) -> &V{
		&self.0
	}
	fn sink(&self) -> &V{
		&self.1
	}
	fn edge(&self) -> &E{
		&self.2
	}
}

impl<V> Edge<V,()> for (V,V)
	where V: Id
{
	fn source(&self) -> &V{
		&self.0
	}
	fn sink(&self) -> &V{
		&self.1
	}
	fn edge(&self) -> &(){
		&()
	}
}

