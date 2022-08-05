fn main() {
    cc::Build::new()
        .file("src/signals/signal_handling.c")
        .warnings(true)
        .compile("signals");
}
