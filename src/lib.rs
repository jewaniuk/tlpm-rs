#[macro_use]
mod macros;

pub mod discovery;
pub mod enums;
pub mod error;

mod correction;
mod filter;
mod interface;
mod io;
mod measure;
mod range;
mod sensor;
mod session;
mod system;

use std::marker::PhantomData;

// suppress warnings for generated c bindings
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// visa boolean constants
pub(crate) const VI_TRUE: sys::ViBoolean = 1;
pub(crate) const VI_FALSE: sys::ViBoolean = 0;

/// Safe wrapper around a Thorlabs power meter session.
///
/// This struct manages the lifetime of the VISA session used to control a Thorlabs
/// power meter. It abstracts the unsafe C-API, providing a safe interface to external Rust users.
pub struct PowerMeter {
    session: sys::ViSession,
    // phantom data tells the rust compiler that this type is not thread safe (!Send and !Sync)
    _marker: PhantomData<*const ()>,
}
