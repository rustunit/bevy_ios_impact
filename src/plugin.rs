use bevy::app::Plugin;

use crate::ImpactResource;

/// bevy plugin: initializes [ImpactResource] to be used in systems etc.
pub struct ImpactPlugin;

impl Plugin for ImpactPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ImpactResource>();
    }
}
