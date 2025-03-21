use bevy::{app::PluginsState, prelude::*};

/// Sets up a [runner](App::set_runner) for the Bevy [application](App) which waits for VBlank
/// between calls to [`update`](App::update).
#[derive(Default)]
pub struct AgbRunnerPlugin;

impl Plugin for AgbRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(|mut app| {
            let vblank = agb::interrupt::VBlank::get();

            while app.plugins_state() == PluginsState::Adding {}

            app.finish();
            app.cleanup();

            loop {
                app.update();

                if let Some(exit) = app.should_exit() {
                    return exit;
                }

                vblank.wait_for_vblank();
            }
        });
    }
}
