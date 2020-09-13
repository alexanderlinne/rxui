use crate::core;

#[derive(new)]
pub struct ObservableSubscribeOn<Observable, Worker>
where
    Worker: core::Worker + Send + 'static,
{
    observable: Observable,
    worker: Worker,
}

impl<Observable, Worker> core::SharedObservable for ObservableSubscribeOn<Observable, Worker>
where
    Observable: core::SharedObservable + Send + 'static,
    Worker: core::Worker + Send + 'static,
{
    type Cancellable = Observable::Cancellable;

    fn actual_subscribe<Observer>(self, observer: Observer)
    where
        Observer: core::Observer<Self::Cancellable, Self::Item, Self::Error> + Send + 'static,
    {
        let observable = self.observable;
        self.worker.schedule(move || {
            observable.actual_subscribe(observer);
        });
    }
}

impl<Observable, Worker> core::Observable for ObservableSubscribeOn<Observable, Worker>
where
    Observable: core::Observable,
    Worker: core::Worker + Send + 'static,
{
    type Item = Observable::Item;
    type Error = Observable::Error;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::scheduler;
    use crate::util::shared::*;

    #[test]
    fn subscribe_on() {
        let scheduler = scheduler::ThreadPoolScheduler::default();
        let test_observer = TestObserver::default();
        vec![0, 1, 2, 3]
            .into_observable()
            .subscribe_on(&scheduler)
            .subscribe(test_observer.clone());
        scheduler.join();
        assert_eq!(test_observer.status(), ObserverStatus::Completed);
        assert_eq!(test_observer.items(), vec![0, 1, 2, 3]);
    }

    #[test]
    fn subscribe_on_shared() {
        let scheduler = scheduler::ThreadPoolScheduler::default();
        let test_observer = TestObserver::default();
        vec![0, 1, 2, 3]
            .into_shared_observable()
            .subscribe_on(&scheduler)
            .subscribe(test_observer.clone());
        scheduler.join();
        assert_eq!(test_observer.status(), ObserverStatus::Completed);
        assert_eq!(test_observer.items(), vec![0, 1, 2, 3]);
    }
}
