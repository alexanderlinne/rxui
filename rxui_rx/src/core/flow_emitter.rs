use crate::flow;
use crate::util::distribute_value;
use std::cmp::min;

pub trait FlowEmitter<Item, Error> {
    fn on_next(&mut self, item: Item);
    fn on_error(&mut self, error: flow::Error<Error>);
    fn on_completed(&mut self);

    fn is_cancelled(&self) -> bool {
        false
    }

    fn requested(&self) -> usize;
}

impl<'o, Item, Error> FlowEmitter<Item, Error> for Box<dyn FlowEmitter<Item, Error> + 'o> {
    fn on_next(&mut self, item: Item) {
        (&mut **self).on_next(item)
    }

    fn on_error(&mut self, error: flow::Error<Error>) {
        (&mut **self).on_error(error)
    }

    fn on_completed(&mut self) {
        (&mut **self).on_completed()
    }

    fn is_cancelled(&self) -> bool {
        (&**self).is_cancelled()
    }

    fn requested(&self) -> usize {
        (&**self).requested()
    }
}

impl<Item, Error> FlowEmitter<Item, Error> for Box<dyn FlowEmitter<Item, Error> + Send + 'static> {
    fn on_next(&mut self, item: Item) {
        (&mut **self).on_next(item)
    }

    fn on_error(&mut self, error: flow::Error<Error>) {
        (&mut **self).on_error(error)
    }

    fn on_completed(&mut self) {
        (&mut **self).on_completed()
    }

    fn is_cancelled(&self) -> bool {
        (&**self).is_cancelled()
    }

    fn requested(&self) -> usize {
        (&**self).requested()
    }
}

impl<'o, Item, Error> FlowEmitter<Item, Error> for Vec<Box<dyn FlowEmitter<Item, Error> + 'o>>
where
    Item: Clone,
    Error: Clone,
{
    fn on_next(&mut self, item: Item) {
        distribute_value(self, |o, i| o.on_next(i), item);
    }

    fn on_error(&mut self, error: flow::Error<Error>) {
        distribute_value(self, |o, e| o.on_error(e), error);
    }

    fn on_completed(&mut self) {
        self.iter_mut().for_each(|o| o.on_completed());
    }

    fn is_cancelled(&self) -> bool {
        self.iter().fold(true, |is_cancelled, item| {
            is_cancelled && item.is_cancelled()
        })
    }

    fn requested(&self) -> usize {
        self.iter().fold(usize::MAX, |min_requested, item| {
            min(min_requested, item.requested())
        })
    }
}

impl<Item, Error> FlowEmitter<Item, Error>
    for Vec<Box<dyn FlowEmitter<Item, Error> + Send + 'static>>
where
    Item: Clone,
    Error: Clone,
{
    fn on_next(&mut self, item: Item) {
        distribute_value(self, |o, i| o.on_next(i), item);
    }

    fn on_error(&mut self, error: flow::Error<Error>) {
        distribute_value(self, |o, e| o.on_error(e), error);
    }

    fn on_completed(&mut self) {
        self.iter_mut().for_each(|o| o.on_completed());
    }

    fn is_cancelled(&self) -> bool {
        self.iter().fold(true, |is_cancelled, item| {
            is_cancelled && item.is_cancelled()
        })
    }

    fn requested(&self) -> usize {
        self.iter().fold(usize::MAX, |min_requested, item| {
            min(min_requested, item.requested())
        })
    }
}