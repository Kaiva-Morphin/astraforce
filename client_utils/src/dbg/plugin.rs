use bevy::prelude::*;

#[derive(Default)]
pub struct DebugOverlayPlugin;

impl Plugin for DebugOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(super::inspector::plugin::SwitchableEguiInspectorPlugin);
    }
}
