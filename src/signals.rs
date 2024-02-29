mod bindings;

use crate::fan_control::FanControl;
use std::{os::raw::c_int, process};

///
/// Sets the callbacks for unix signals.
///
/// TODO: maybe strerror?
///
pub fn listen() -> Result<(), ()> {
    if unsafe { bindings::signals_listen(signal_handler) } {
        Ok(())
    } else {
        Err(())
    }
}

///
/// Signal handler function that enables automatic control and exits. The
/// reason why we need to exit here is because otherwise the system would have
/// to wait for sleeping in the main thread to end before closing, which could
/// potentially slow down powering off the system.
///
extern "C" fn signal_handler(_: c_int) {
    FanControl::disable();

    // TODO: remove in future and exit from main to clean up other resources
    process::exit(0);
}
