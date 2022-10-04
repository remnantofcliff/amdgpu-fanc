use crate::RUNNING;
use std::{os::raw::c_int, sync::atomic};

///
/// Sets the callbacks for unix signals.
///
pub fn listen() {
    unsafe { signals_listen(signal_handler) }
}

///
/// Signal handler function
///
extern "C" fn signal_handler(_: c_int) {
    RUNNING.store(false, atomic::Ordering::Relaxed);
}

extern "C" {
    pub fn signals_listen(handler: extern "C" fn(c_int));
}
