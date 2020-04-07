//! ECS audio bundles

use amethyst_assets::Processor;
use amethyst_core::{
    bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    SystemDesc,
};
use amethyst_error::Error;

use crate::{source::*, systems::AudioSystemDesc};

/// Audio bundle
///
/// This will only add the audio system and the asset processor for `Source`.
///
/// `DjSystem` must be added separately if you want to use our background music system.
///
/// The generic N type should be the same as the one in `Transform`.
#[derive(Default, Debug)]
pub struct AudioBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for AudioBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add_thread_local(AudioSystemDesc::new().build(world));
        builder.add_thread_local(Processor::<Source>::new());
        Ok(())
    }
}
