pub mod signals {
    extern "C" {
        #[link_name = "\u{1}_ZN7signals6listenEv"]
        pub fn listen() -> i8;
        #[link_name = "\u{1}_ZN7signals12should_closeEv"]
        pub fn should_close() -> bool;
    }
}
