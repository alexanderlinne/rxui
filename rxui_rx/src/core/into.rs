use crate::core;
use crate::flow;
use crate::marker;

pub trait IntoObservable {
    type Observable: core::Observable;

    fn into_observable(self) -> Self::Observable;
}

pub trait IntoSharedObservable {
    type Observable: core::SharedObservable;

    fn into_shared_observable(self) -> marker::Shared<Self::Observable>;
}

impl<T> IntoSharedObservable for T
where
    T: IntoObservable,
    T::Observable: core::SharedObservable,
{
    type Observable = T::Observable;

    fn into_shared_observable(self) -> marker::Shared<Self::Observable> {
        use crate::core::SharedObservable;
        self.into_observable().into_shared()
    }
}

pub trait IntoFlow {
    type Flow: core::Flow;

    fn into_flow(self, strategy: flow::BackpressureStrategy) -> marker::Flow<Self::Flow>;
}

pub trait IntoSharedFlow {
    type Flow: core::SharedFlow;

    fn into_shared_flow(
        self,
        strategy: flow::BackpressureStrategy,
    ) -> marker::Shared<marker::Flow<Self::Flow>>;
}

impl<T> IntoSharedFlow for T
where
    T: IntoFlow,
    T::Flow: core::SharedFlow,
{
    type Flow = T::Flow;

    fn into_shared_flow(
        self,
        strategy: flow::BackpressureStrategy,
    ) -> marker::Shared<marker::Flow<Self::Flow>> {
        self.into_flow(strategy).into_shared()
    }
}

pub trait IntoObservableEmitter<'o, Item, Error> {
    type Emitter: core::ObservableEmitter<Item, Error>;

    fn into_emitter(self) -> Self::Emitter;
}

pub trait IntoSharedObservableEmitter<Item, Error> {
    type Emitter: core::ObservableEmitter<Item, Error>;

    fn into_shared_emitter(self) -> Self::Emitter;
}

pub trait IntoFlowEmitter<'o, Item, Error> {
    type Emitter: core::FlowEmitter<Item, Error>;

    fn into_emitter(self, strategy: flow::BackpressureStrategy) -> Self::Emitter;
}

pub trait IntoSharedFlowEmitter<Item, Error> {
    type Emitter: core::FlowEmitter<Item, Error>;

    fn into_shared_emitter(self, strategy: flow::BackpressureStrategy) -> Self::Emitter;
}
