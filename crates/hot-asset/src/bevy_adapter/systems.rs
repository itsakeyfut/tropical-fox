use bevy::asset::AssetEvent;
use bevy::prelude::*;
use std::any::type_name;

use super::events::AssetReloaded;
use super::resources::{HotAssetHandle, ReloadMetrics, VerboseLogging};

/// Monitor asset changes and emit events
pub fn handle_asset_changes<T>(
    mut events: MessageWriter<AssetReloaded<T>>,
    mut asset_events: MessageReader<AssetEvent<T>>,
    handle: Res<HotAssetHandle<T>>,
    assets: Res<Assets<T>>,
    mut metrics: Option<ResMut<ReloadMetrics<T>>>,
    verbose: Res<VerboseLogging>,
) where
    T: Asset + TypePath,
{
    for event in asset_events.read() {
        match event {
            AssetEvent::LoadedWithDependencies { id } if *id == handle.0.id() => {
                // Successfully loaded/reloaded
                if assets.get(&handle.0).is_some() {
                    if verbose.0 {
                        info!("✅ Successfully reloaded config: {}", type_name::<T>());
                    }

                    // Update metrics
                    if let Some(ref mut m) = metrics {
                        m.record_success();
                    }

                    // Emit success event
                    events.write(AssetReloaded::success(handle.0.clone()));
                }
            }
            // Note: AssetEvent in Bevy 0.17 doesn't have a Failed variant
            // Errors are logged internally by the asset system
            _ => {}
        }
    }
}
