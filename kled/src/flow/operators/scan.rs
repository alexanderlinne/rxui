use crate::core;
use crate::flow;

#[operator(type = "flow", item = "ItemOut")]
pub struct Scan<ItemOut, BinaryOp>
where
    ItemOut: Clone,
    BinaryOp: FnMut(ItemOut, Item) -> ItemOut,
{
    initial_value: ItemOut,
    binary_op: BinaryOp,
}

#[derive(new)]
struct ScanSubscriber<Subscriber, ItemOut, BinaryOp> {
    subscriber: Subscriber,
    previous_value: ItemOut,
    binary_op: BinaryOp,
}

impl<Subscription, ItemIn, Subscriber, ItemOut, Error, BinaryOp>
    core::Subscriber<Subscription, ItemIn, Error> for ScanSubscriber<Subscriber, ItemOut, BinaryOp>
where
    Subscriber: core::Subscriber<Subscription, ItemOut, Error>,
    BinaryOp: FnMut(ItemOut, ItemIn) -> ItemOut,
    ItemOut: Clone,
{
    fn on_subscribe(&mut self, cancellable: Subscription) {
        self.subscriber.on_subscribe(cancellable);
        self.subscriber.on_next(self.previous_value.clone());
    }
    fn on_next(&mut self, item: ItemIn) {
        self.previous_value = (self.binary_op)(self.previous_value.clone(), item);
        self.subscriber.on_next(self.previous_value.clone());
    }
    fn on_error(&mut self, error: flow::Error<Error>) {
        self.subscriber.on_error(error);
    }
    fn on_completed(&mut self) {
        self.subscriber.on_completed();
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::subscriber::*;

    #[test]
    fn local_scan() {
        let test_subscriber = TestSubscriber::default();
        vec![0, 1, 2, 3]
            .into_flow()
            .scan(0, |a, b| a + b)
            .subscribe(test_subscriber.clone());

        assert_eq!(test_subscriber.status(), SubscriberStatus::Completed);
        assert_eq!(test_subscriber.items(), vec![0, 0, 1, 3, 6]);
    }
}
