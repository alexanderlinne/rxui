use crate::cancellable::{local, shared};
use crate::core;
use crate::core::{IntoObservableEmitter, IntoSharedObservableEmitter, ObservableEmitter};
use crate::util;

#[doc(hidden)]
pub struct IntoIterObservable<IntoIter> {
    iterable: IntoIter,
}

impl<IntoIter> IntoIterObservable<IntoIter>
where
    IntoIter: IntoIterator,
{
    fn new(iterable: IntoIter) -> Self {
        Self { iterable }
    }
}

impl<IntoIter> core::Observable for IntoIterObservable<IntoIter>
where
    IntoIter: IntoIterator,
{
    type Item = IntoIter::Item;
    type Error = util::Infallible;
}

impl<'o, IntoIter> core::LocalObservable<'o> for IntoIterObservable<IntoIter>
where
    IntoIter: IntoIterator,
{
    type Cancellable = local::BoolCancellable;

    fn actual_subscribe<Observer>(self, observer: Observer)
    where
        Observer: core::Observer<Self::Cancellable, Self::Item, Self::Error> + 'o,
    {
        let mut observer = observer.into_emitter();
        for v in self.iterable.into_iter() {
            if !observer.is_cancelled() {
                observer.on_next(v);
            } else {
                break;
            }
        }
        if !observer.is_cancelled() {
            observer.on_completed();
        }
    }
}

impl<IntoIter> core::SharedObservable for IntoIterObservable<IntoIter>
where
    IntoIter: IntoIterator,
{
    type Cancellable = shared::BoolCancellable;

    fn actual_subscribe<Observer>(self, observer: Observer)
    where
        Observer: core::Observer<Self::Cancellable, Self::Item, Self::Error> + Send + 'static,
    {
        let mut observer = observer.into_shared_emitter();
        for v in self.iterable.into_iter() {
            if !observer.is_cancelled() {
                observer.on_next(v);
            } else {
                break;
            }
        }
        if !observer.is_cancelled() {
            observer.on_completed();
        }
    }
}

impl<IntoIter> core::IntoObservable for IntoIter
where
    IntoIter: IntoIterator,
{
    type Observable = IntoIterObservable<IntoIter>;

    fn into_observable(self) -> Self::Observable {
        IntoIterObservable::new(self)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn iterable_into_observable() {
        let vec = vec![0, 1, 2, 3];
        let sum = Rc::new(RefCell::new(0));
        let sum_move = sum.clone();
        vec.into_observable()
            .subscribe_next(move |v| (*sum_move.borrow_mut()) += v);
        assert_eq!(*sum.borrow(), 6);
    }
}
