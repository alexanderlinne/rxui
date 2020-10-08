use crate::cancellable::*;
use crate::core;
use std::marker::PhantomData;

pub struct FromObserver<Observer, Item, Error> {
    observer: Observer,
    stub: BoolCancellableStub,
    phantom: PhantomData<(Item, Error)>,
}

impl<Observer, Item, Error> FromObserver<Observer, Item, Error>
where
    Observer: core::Observer<BoolCancellable, Item, Error>,
{
    pub fn new(mut observer: Observer) -> Self {
        use crate::core::CancellableProvider;
        let stub = BoolCancellableStub::default();
        observer.on_subscribe(stub.cancellable());
        Self {
            observer,
            stub,
            phantom: PhantomData,
        }
    }
}

impl<ObserverType, Item, Error> core::ObservableEmitter<Item, Error>
    for FromObserver<ObserverType, Item, Error>
where
    ObserverType: core::Observer<BoolCancellable, Item, Error>,
{
    fn on_next(&mut self, item: Item) {
        self.observer.on_next(item);
    }

    fn on_error(&mut self, error: Error) {
        self.observer.on_error(error);
    }

    fn on_completed(&mut self) {
        self.observer.on_completed();
    }

    fn is_cancelled(&self) -> bool {
        self.stub.is_cancelled()
    }
}

impl<Observer, Item, Error> core::IntoObservableEmitter<Item, Error> for Observer
where
    Observer: core::Observer<BoolCancellable, Item, Error> + Send + 'static,
{
    type Emitter = FromObserver<Observer, Item, Error>;

    fn into_emitter(self) -> Self::Emitter {
        FromObserver::new(self)
    }
}