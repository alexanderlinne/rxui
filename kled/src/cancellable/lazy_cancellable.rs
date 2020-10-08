use crate::core;
use crate::sync::{Arc, Mutex};

pub struct LazyCancellableStub<Cancellable> {
    data: Arc<Mutex<Data<Cancellable>>>,
}

impl<Cancellable> Default for LazyCancellableStub<Cancellable> {
    fn default() -> Self {
        Self {
            data: Arc::new(Mutex::new(Data::default())),
        }
    }
}

impl<Cancellable> core::CancellableProvider for LazyCancellableStub<Cancellable>
where
    Cancellable: core::Cancellable,
{
    type Cancellable = LazyCancellable<Cancellable>;

    fn cancellable(&self) -> LazyCancellable<Cancellable> {
        LazyCancellable {
            data: self.data.clone(),
        }
    }
}

impl<Cancellable> LazyCancellableStub<Cancellable>
where
    Cancellable: core::Cancellable,
{
    pub fn set_cancellable(&mut self, cancellable: Cancellable) {
        let mut data = self.data.lock();
        if data.cancelled {
            cancellable.cancel();
        }
        data.subscription = Some(cancellable);
    }
}

#[derive(Clone)]
pub struct LazyCancellable<Cancellable> {
    data: Arc<Mutex<Data<Cancellable>>>,
}

impl<Cancellable> core::Cancellable for LazyCancellable<Cancellable>
where
    Cancellable: core::Cancellable,
{
    fn cancel(&self) {
        let mut data = self.data.lock();
        if let Some(subscription) = &data.subscription {
            subscription.cancel();
        } else {
            data.cancelled = true;
        }
    }
}

struct Data<Cancellable> {
    cancelled: bool,
    subscription: Option<Cancellable>,
}

impl<Cancellable> Default for Data<Cancellable> {
    fn default() -> Self {
        Self {
            cancelled: false,
            subscription: None,
        }
    }
}