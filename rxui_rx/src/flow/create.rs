use crate::core;
use crate::core::{IntoFlowEmitter, IntoSharedFlowEmitter};
use crate::marker;
use crate::subscription::*;
use std::marker::PhantomData;

#[derive(new, Clone)]
#[doc(hidden)]
pub struct FlowCreate<F, Item, Error> {
    emitter_consumer: F,
    phantom: PhantomData<(Item, Error)>,
}

impl<'o, F, Item, Error> core::LocalFlow<'o> for FlowCreate<F, Item, Error>
where
    F: FnOnce(Box<dyn core::FlowEmitter<Item, Error> + 'o>),
    Item: 'o,
    Error: 'o,
{
    type Subscription = local::AccumulateSubscription;

    fn actual_subscribe<Subscriber>(self, subscriber: Subscriber)
    where
        Subscriber: core::Subscriber<Self::Subscription, Self::Item, Self::Error> + 'o,
    {
        let emitter = subscriber.into_emitter();
        (self.emitter_consumer)(Box::new(emitter));
    }
}

impl<F, Item, Error> core::SharedFlow for FlowCreate<F, Item, Error>
where
    F: FnOnce(Box<dyn core::FlowEmitter<Item, Error> + Send>),
    Item: Send + 'static,
    Error: Send + 'static,
{
    type Subscription = shared::AccumulateSubscription;

    fn actual_subscribe<Subscriber>(self, subscriber: Subscriber)
    where
        Subscriber: core::Subscriber<Self::Subscription, Self::Item, Self::Error> + Send + 'static,
    {
        let emitter = subscriber.into_shared_emitter();
        (self.emitter_consumer)(Box::new(emitter));
    }
}

impl<F, Item, Error> core::Flow for FlowCreate<F, Item, Error> {
    type Item = Item;
    type Error = Error;
}

pub fn create<F, Item, Error>(emitter_consumer: F) -> marker::Flow<FlowCreate<F, Item, Error>> {
    marker::Flow::new(FlowCreate::new(emitter_consumer))
}
