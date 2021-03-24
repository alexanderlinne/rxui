use crate::{core, flow, util};
use crate::signal::Signal;
use async_trait::async_trait;

#[operator(
    type = "flow",
    subscription = "util::Never",
    item = "Signal<Subscription, Item, flow::Error<Error>>",
    error = "util::Never"
)]
pub struct Materialize {}

#[derive(new)]
struct MaterializeSubscriber<Subscriber> {
    subscriber: Subscriber,
}

#[async_trait]
impl<Subscription, Subscriber, Item, Error>
    core::Subscriber<Subscription, Item, Error> for MaterializeSubscriber<Subscriber>
where
    Subscriber: core::Subscriber<util::Never, Signal<Subscription, Item, flow::Error<Error>>, util::Never> + Send,
    Subscription: Send + 'static,
    Item: Send + 'static,
    Error: Send + 'static,
{
    async fn on_subscribe(&mut self, subscription: Subscription) {
        self.subscriber.on_next(Signal::Subscribe(subscription)).await;
    }
    async fn on_next(&mut self, item: Item) {
        self.subscriber.on_next(Signal::Item(item)).await;
    }
    async fn on_error(&mut self, error: flow::Error<Error>) {
        self.subscriber.on_next(Signal::Error(error)).await;
    }
    async fn on_completed(&mut self) {
        self.subscriber.on_next(Signal::Completed).await;
        self.subscriber.on_completed().await;
    }
}
