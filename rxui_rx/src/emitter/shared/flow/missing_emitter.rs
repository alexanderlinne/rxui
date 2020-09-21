use crate::core;
use crate::flow;
use crate::subscription::shared::*;
use std::marker::PhantomData;

pub struct MissingEmitter<Subscriber, Item, Error> {
    subscriber: Subscriber,
    stub: IgnoreSubscriptionStub,
    phantom: PhantomData<(Item, Error)>,
}

impl<Subscriber, Item, Error> MissingEmitter<Subscriber, Item, Error>
where
    Subscriber: core::Subscriber<Box<dyn core::Subscription + Send + 'static>, Item, Error>
        + Send
        + 'static,
{
    pub fn new(mut subscriber: Subscriber) -> Self {
        let stub = IgnoreSubscriptionStub::default();
        subscriber.on_subscribe(Box::new(stub.subscription()));
        Self {
            subscriber,
            stub,
            phantom: PhantomData,
        }
    }
}

impl<Subscriber, Item, Error> core::FlowEmitter<Item, Error>
    for MissingEmitter<Subscriber, Item, Error>
where
    Subscriber: core::Subscriber<Box<dyn core::Subscription + Send + 'static>, Item, Error>
        + Send
        + 'static,
{
    fn on_next(&mut self, item: Item) {
        self.subscriber.on_next(item);
    }

    fn on_error(&mut self, error: Error) {
        self.subscriber.on_error(flow::Error::Upstream(error));
    }

    fn on_completed(&mut self) {
        self.subscriber.on_completed();
    }

    fn is_cancelled(&self) -> bool {
        self.stub.is_cancelled()
    }
}
