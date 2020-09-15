use crate::core;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct BoolSubscriptionStub {
    data: Arc<Data>,
}

impl Default for BoolSubscriptionStub {
    fn default() -> Self {
        Self {
            data: Arc::new(Data {
                cancelled: AtomicBool::new(false),
                requested: AtomicUsize::new(0),
            }),
        }
    }
}

impl BoolSubscriptionStub {
    pub fn subscription(&self) -> BoolSubscription {
        BoolSubscription {
            data: self.data.clone(),
        }
    }

    pub fn requested(&self) -> usize {
        self.data.requested.swap(0, Ordering::SeqCst)
    }

    pub fn is_cancelled(&self) -> bool {
        self.data.cancelled.load(Ordering::Acquire)
    }
}

#[derive(Clone)]
pub struct BoolSubscription {
    data: Arc<Data>,
}

impl core::Subscription for BoolSubscription {
    fn cancel(&self) {
        self.data.cancelled.store(true, Ordering::Release);
    }

    fn is_cancelled(&self) -> bool {
        self.data.cancelled.load(Ordering::Acquire)
    }

    fn request(&self, count: usize) {
        self.data.requested.fetch_add(count, Ordering::SeqCst);
    }
}

struct Data {
    cancelled: AtomicBool,
    requested: AtomicUsize,
}