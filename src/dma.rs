use bevy::prelude::*;

/// Sets up the DMA subsystem.
#[derive(Default)]
pub struct AgbDmaPlugin;

impl Plugin for AgbDmaPlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        let Some(dma_controller) = app
            .world_mut()
            .remove_non_send_resource::<agb::dma::DmaController>()
        else {
            return;
        };

        app.insert_resource(DmaController(dma_controller));
    }
}

/// Manages access to the Game Boy Advance's DMA
#[derive(Resource, Deref, DerefMut)]
pub struct DmaController(agb::dma::DmaController);
