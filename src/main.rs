use crate::hwmon_files::HwmonFiles;
use interpolation::TempToPwm;
use std::{
    os::raw::c_int,
    sync::atomic::{self, AtomicBool},
    time::Duration,
};

mod hwmon_files;
mod interpolation;
mod signals;

const SLEEP_DURATION: Duration = Duration::from_secs(5);

static SHOULD_CLOSE: AtomicBool = AtomicBool::new(false);

///
/// Signal handler function
///
extern "C" fn signal_handler(_: c_int) {
    SHOULD_CLOSE.store(true, atomic::Ordering::Relaxed);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // May panic during here, otherwise the program should never realistically
    // panic
    let temp_to_pwm = TempToPwm::from_args();

    // Get necessary files
    let mut files = HwmonFiles::new()?;

    // Set listener for shutdown signals
    signals::listen(signal_handler)?;

    // Avoid allocations by reusing buffer
    let mut string_buffer = String::with_capacity(8);

    loop {
        if SHOULD_CLOSE.load(atomic::Ordering::Relaxed) {
            break Ok(());
        }

        files.update_pwms(&mut string_buffer, &temp_to_pwm)?;

        std::thread::sleep(SLEEP_DURATION);
    }
}
