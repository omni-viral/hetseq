use {List, Queue};
pub use num::{Num, P, S, Z};

pub trait Length {
    type Length: Num;
    #[inline]
    fn len() -> usize { Self::Length::value() }
}

impl Length for List<()> {
    type Length = Z;
}

impl<H, T> Length for List<(H, List<T>)>
    where List<T>: Length
{
    type Length = S<<List<T> as Length>::Length>;
}

impl Length for Queue<()> {
    type Length = Z;
}

impl<H, T> Length for Queue<(Queue<H>, T)>
    where Queue<H>: Length
{
    type Length = S<<Queue<H> as Length>::Length>;
}