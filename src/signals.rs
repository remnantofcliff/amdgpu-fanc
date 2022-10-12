use crate::fan_control::FanControl;
use std::{os::raw::c_int, process};

///
/// Sets the callbacks for unix signals.
///
pub fn listen() {
    unsafe { signals_listen(signal_handler) }
}

///
/// Signal handler function that enables automatic control and exits.
///
extern "C" fn signal_handler(_: c_int) {
    FanControl::disable();

    process::exit(0);
}

extern "C" {
    pub fn signals_listen(handler: extern "C" fn(c_int));
}
