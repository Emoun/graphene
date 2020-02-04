/// Trait alias
pub trait Id: Copy + Eq
{
}
impl<T> Id for T where T: Copy + Eq {}
