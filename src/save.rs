use agb::save::MediaType;
use bevy::prelude::*;

/// Sets up a [`SaveManager`] using the provided [`SaveType`] if any.
#[derive(Default)]
pub struct AgbSavePlugin {
    /// Specify the [type of media](MediaType) used to load/store save data with.
    pub save_type: Option<MediaType>,
}

impl Plugin for AgbSavePlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        use MediaType::*;

        let Some(mut save_manager) = app
            .world_mut()
            .remove_non_send_resource::<agb::save::SaveManager>()
        else {
            return;
        };

        match self.save_type {
            Some(Sram32K) => save_manager.init_sram(),
            Some(Flash64K) => save_manager.init_flash_64k(),
            Some(Flash128K) => save_manager.init_flash_128k(),
            Some(Eeprom512B) => save_manager.init_eeprom_512b(),
            Some(Eeprom8K) => save_manager.init_eeprom_8k(),
            _ => {}
        }

        app.insert_resource(SaveManager(save_manager));
    }
}

/// Manages access to the Game Boy Advance cartridge's save chip.
/// Initialization is handled by [`AgbSavePlugin`].
/// See [`access`](agb::save::SaveManager::access) and
/// [`access_with_timer`](agb::save::SaveManager::access_with_timer).
#[derive(Resource, Deref, DerefMut)]
pub struct SaveManager(agb::save::SaveManager);
