use {IntoQueue, Queue};

/// Heterogenous list
/// Supports pushing, splitting to head and tail
/// Mapping and folding
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct List<L>(pub L);
impl List<()> {
    #[inline]
    pub fn new() -> List<()> {
        List(())
    }
}

impl<L> List<L> {
    #[inline]
    pub fn push<V>(self, value: V) -> List<(V, List<L>)> {
        List((value, self))
    }
}

impl<H, T> List<(H, List<T>)> {
    #[inline]
    pub fn head(&self) -> &H {
        let List((ref head, _)) = *self;
        head
    }
    #[inline]
    pub fn tail(&self) -> &List<T> {
        let List((_, ref tail)) = *self;
        tail
    }
}

pub trait IntoList {
    ///
    type List;
    ///
    fn into_list(self) -> Self::List;
}

impl<L> IntoList for List<L> {
    type List = Self;
    fn into_list(self) -> Self { self }
}

pub trait IntoQueueImpl<L> {
    type Queue;
    fn into_queue_impl(self, L) -> Self::Queue;
}

impl<H, T, Q> IntoQueueImpl<Q> for List<(H, T)>
    where T: IntoQueueImpl<Queue<(Q, H)>>
{
    type Queue = T::Queue;

    fn into_queue_impl(self, queue: Q) -> Self::Queue {
        let List((head, tail)) = self;
        let queue = Queue((queue, head));
        tail.into_queue_impl(queue)
    }
}

impl<L> IntoQueue for L
    where L: IntoQueueImpl<Queue<()>>
{
    type Queue = <L as IntoQueueImpl<Queue<()>>>::Queue;
    fn into_queue(self) -> Self::Queue {
        self.into_queue_impl(Queue::new())
    }
}
