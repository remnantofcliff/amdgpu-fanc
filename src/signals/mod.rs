mod bindings;

///
/// Sets the callbacks for unix signals.
///
pub fn listen() -> Result<(), &'static str> {
    if unsafe { bindings::signals::listen() } == 0 {
        Ok(())
    } else {
        Err("Failed to set signals")
    }
}

///
/// Returns if the program should close.
///
pub fn should_close() -> bool {
    unsafe { bindings::signals::should_close() }
}
