fn main() {
    cc::Build::new()
        .file("src/signals/signal_handling.cpp")
        .warnings(true)
        .compile("signals");
}
