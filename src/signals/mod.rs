use std::os::raw::c_int;

mod bindings;

///
/// Sets the callbacks for unix signals.
///
pub fn listen(handler: extern "C" fn(c_int)) -> Result<(), &'static str> {
    if unsafe { bindings::signals_listen(handler) } == 0 {
        Ok(())
    } else {
        Err("Failed to set signal callbacks")
    }
}
