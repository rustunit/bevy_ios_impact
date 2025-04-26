use bevy_app::{App, Plugin};

use crate::ImpactResource;

/// bevy plugin: initializes [ImpactResource] to be used in systems etc.
pub struct ImpactPlugin;

impl Plugin for ImpactPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ImpactResource>();
    }
}
