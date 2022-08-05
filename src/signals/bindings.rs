use std::os::raw::c_int;

extern "C" {
    ///
    /// Sets handler for signals that should quit the program. Returns 0 on
    /// success or a negative value to indicate error.
    ///
    pub fn signals_listen(handler: extern "C" fn(c_int)) -> i8;
}
