use crate::core;
use crate::flow;
use crate::marker;

#[derive(Clone)]
pub struct Flow<Actual> {
    pub(crate) actual: Actual,
}

impl<Actual> Flow<Actual> {
    pub fn new(actual: Actual) -> Self {
        Self { actual }
    }

    pub fn into_shared(self) -> marker::Shared<Self>
    where
        Self: Sized,
    {
        marker::Shared::new(self)
    }
}

impl<Cancellable, Item, Error, T> core::Subscriber<Cancellable, Item, Error> for Flow<T>
where
    T: core::Subscriber<Cancellable, Item, Error>,
{
    fn on_subscribe(&mut self, cancellable: Cancellable) {
        self.actual.on_subscribe(cancellable);
    }

    fn on_next(&mut self, item: Item) {
        self.actual.on_next(item);
    }

    fn on_error(&mut self, error: flow::Error<Error>) {
        self.actual.on_error(error);
    }

    fn on_completed(&mut self) {
        self.actual.on_completed();
    }
}