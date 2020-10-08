//! [![github]](https://github.com/alexanderlinne/kled)&ensp;
//!
//! [github]: https://img.shields.io/github/workflow/status/alexanderlinne/kled/CI?style=for-the-badge&logo=github
//!
#![deny(intra_doc_link_resolution_failure)]

#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate kled_derive;
extern crate num_cpus;
extern crate parking_lot;

mod mock;
use mock::*;

pub mod cancellable;
pub mod core;
pub mod emitter;
pub mod flow;
pub mod observable;
pub mod observer;
pub mod operators;
pub mod scheduler;
pub mod subject;
pub mod subscriber;
pub mod subscription;
pub mod util;

#[doc(hidden)]
pub mod prelude;