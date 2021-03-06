use crate::{core, observer};
use crate::cancellable::LazyCancellable;
use crate::observable::operators::*;
use async_trait::async_trait;

/// A non-backpressured source of `Item`s to which an [`Observer`] may subscribe.
///
/// [`Observable`] is the base trait which any observable type must implement. It defines the
/// type of `Item`s and `Error`s it may emit and the `Cancellable` type the observable passes
/// to the [`Observer`] via [`Observer::on_subscribe`].
///
/// [`Observer`]: trait.Observer.html
/// [`ObservableExt`]: trait.ObservableExt.html
/// [`Observer::on_subscribe`]: trait.Observer.html#tymethod.on_subscribe
#[async_trait]
pub trait Observable<Cancellable, Item, Error>
where
    Cancellable: core::Cancellable + Send + Sync + 'static,
    Item: Send + 'static,
    Error: Send + 'static,
{
    async fn subscribe<Observer>(self, observer: Observer)
    where
        Observer: core::Observer<Cancellable, Item, Error> + Send + 'static;

    async fn subscribe_next<NextFn>(self, next_fn: NextFn) -> LazyCancellable<Cancellable>
    where
        Self: Sized,
        NextFn: FnMut(Item) + Send + 'static,
    {
        let observer = observer::LambdaObserver::new(
            next_fn,
            |_| {
                panic! {}
            },
            || {},
        );
        let cancellable = observer.cancellable();
        self.subscribe(observer).await;
        cancellable
    }

    async fn subscribe_all<NextFn, ErrorFn, CompletedFn>(
        self,
        next_fn: NextFn,
        error_fn: ErrorFn,
        complete_fn: CompletedFn,
    ) -> LazyCancellable<Cancellable>
    where
        Self: Sized,
        NextFn: FnMut(Item) + Send + 'static,
        ErrorFn: FnMut(Error) + Send + 'static,
        CompletedFn: FnMut() + Send + 'static,
    {
        let observer = observer::LambdaObserver::new(next_fn, error_fn, complete_fn);
        let cancellable = observer.cancellable();
        self.subscribe(observer).await;
        cancellable
    }

    fn dematerialize(
        self,
    ) -> Dematerialize<Self, Cancellable, Item, Error>
    where
        Self: Sized,
    {
        Dematerialize::new(self)
    }

    /// Returns an [`Observable`] that applies the function `unary_op` to each element of the
    /// current `Observable` and emits the results of those function calls.
    ///
    /// [`Observable`]: trait.Observable.html
    fn map<ItemOut, UnaryOp>(
        self,
        unary_op: UnaryOp,
    ) -> Map<Self, Cancellable, Item, Error, ItemOut, UnaryOp>
    where
        Self: Sized,
        UnaryOp: FnMut(Item) -> ItemOut + Send + 'static,
    {
        Map::new(self, unary_op)
    }

    fn materialize(
        self,
    ) -> Materialize<Self, Cancellable, Item, Error>
    where
        Self: Sized,
    {
        Materialize::new(self)
    }

    /// Returns an [`Observable`] that performs the current `Observable`'s emissions on the
    /// specified [`Scheduler`]. Note that `onError` notifications will not be sent in order
    /// i.e. not all items sent before the error may be re-emitted on the scheduler.
    ///
    /// [`Observable`]: trait.Observable.html
    /// [`Scheduler`]: trait.Scheduler.html
    fn observe_on<Scheduler>(
        self,
        scheduler: Scheduler,
    ) -> ObserveOn<Self, Cancellable, Item, Error, Scheduler>
    where
        Self: Sized,
        Scheduler: core::Scheduler + Send + 'static,
    {
        Dematerialize::new(ObserveOnRaw::new(Materialize::new(self), scheduler))
    }

    /// Returns an [`Observable`] that first emits the provided `initial_value` as an item and the
    /// emits one item for each item emitted by the current `Observable`. Each of those emissions
    /// is the result of appying `binary_op` to the previous emission and the item received from
    /// the current `Observable`.
    ///
    /// [`Observable`]: trait.Observable.html
    fn scan<ItemOut, BinaryOp>(
        self,
        initial_value: ItemOut,
        binary_op: BinaryOp,
    ) -> Scan<Self, Cancellable, Item, Error, ItemOut, BinaryOp>
    where
        Self: Sized,
        ItemOut: Clone + Send + 'static,
        BinaryOp: FnMut(ItemOut, Item) -> ItemOut + Send + 'static,
    {
        Scan::new(self, initial_value, binary_op)
    }

    /// Asynchronously subscribes [`Observer`]s to the current [`Observable`] on the given
    /// [`Scheduler`].
    ///
    /// [`Observable`]: trait.Observable.html
    /// [`Observer`]: trait.Observer.html
    /// [`Scheduler`]: trait.Scheduler.html
    fn subscribe_on<Scheduler>(
        self,
        scheduler: Scheduler,
    ) -> SubscribeOn<Self, Cancellable, Item, Error, Scheduler>
    where
        Self: Sized,
        Scheduler: core::Scheduler + Send + 'static,
    {
        SubscribeOn::new(self, scheduler)
    }
}
