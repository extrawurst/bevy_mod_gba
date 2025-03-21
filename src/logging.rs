use agb::mgba::{DebugLevel, Mgba};
use bevy::prelude::*;
use log::{Level, LevelFilter, Metadata, Record};

/// Integrates logging with the MGBA emulator.
#[derive(Default)]
pub struct AgbLogPlugin;

impl Plugin for AgbLogPlugin {
    fn build(&self, _app: &mut App) {
        #[expect(unsafe_code, reason = "setting up logging requires unsafe code")]
        // SAFETY: Plugin::build should only be called once at a time
        unsafe {
            if let Ok(()) = log::set_logger_racy(&MgbaLogger) {
                log::set_max_level_racy(LevelFilter::Info);
            }
        }
    }
}

struct MgbaLogger;

impl log::Log for MgbaLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Some(mut mgba) = Mgba::new() {
                let level = match record.level() {
                    Level::Error => Some(DebugLevel::Error),
                    Level::Warn => Some(DebugLevel::Warning),
                    Level::Info => Some(DebugLevel::Info),
                    Level::Debug => Some(DebugLevel::Debug),
                    Level::Trace => None,
                };

                if let Some(level) = level {
                    let _ = mgba.print(*record.args(), level);
                }
            }
        }
    }

    fn flush(&self) {}
}
