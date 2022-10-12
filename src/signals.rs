mod bindings;

use crate::fan_control::FanControl;
use std::{os::raw::c_int, process};

///
/// Sets the callbacks for unix signals.
///
pub fn listen() {
    unsafe { bindings::signals_listen(signal_handler) }
}

///
/// Signal handler function that enables automatic control and exits. The
/// reason why we need to exit here is because otherwise the system would have
/// to wait for sleeping in the main thread to end before closing.
///
extern "C" fn signal_handler(_: c_int) {
    FanControl::disable();

    process::exit(0);
}
