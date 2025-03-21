//! A bare-bones example showing minimal setup.

#![no_std]
#![no_main]

use bevy::prelude::*;
use bevy_mod_gba::prelude::*;

/// Main entry point.
#[expect(unsafe_code)]
#[unsafe(export_name = "main")]
pub extern "C" fn main() -> ! {
    App::new().add_plugins(AgbPlugin).run();

    loop {}
}
