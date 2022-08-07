use std::os::raw::c_int;

extern "C" {
    pub fn signals_listen(handler: extern "C" fn(c_int)) -> c_int;
}
