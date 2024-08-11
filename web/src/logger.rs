use std::time::Duration;

use leptos::*;
use log::Level;

pub struct Logger {
    timeout: u64,
    show: RwSignal<bool>,
    level: RwSignal<log::Level>,
    message: RwSignal<String>,
}

impl Logger {
    pub fn new(
        timeout: u64,
        show: RwSignal<bool>,
        level: RwSignal<Level>,
        message: RwSignal<String>,
    ) -> Self {
        Self {
            timeout,
            show,
            level,
            message,
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Warn
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            self.level.set(record.level());
            self.message
                .set(format!("{}: {}", record.level(), record.args()));

            let show = self.show;
            show.set(true);
            set_timeout(
                move || {
                    show.set(false);
                },
                Duration::from_millis(self.timeout),
            );
        }
    }

    fn flush(&self) {}
}
