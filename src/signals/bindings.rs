use std::os::raw::c_int;

extern "C" {
    #[must_use]
    pub fn signals_listen(handler: extern "C" fn(c_int)) -> bool;
}
