use bevy::prelude::*;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

use super::events::AssetReloaded;
use super::loader::RonAssetLoader;
use super::resources::{HotAssetHandle, ReloadMetrics, VerboseLogging};
use super::systems::handle_asset_changes;

pub struct HotReloadPlugin<T: Asset + DeserializeOwned> {
    path: String,
    verbose: bool,
    condition: Option<Box<dyn Fn() -> bool + Send + Sync>>,
    enable_metrics: bool,
    _phantom: PhantomData<T>,
}

impl<T: Asset + DeserializeOwned> HotReloadPlugin<T> {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            verbose: false,
            condition: None,
            enable_metrics: false,
            _phantom: PhantomData,
        }
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn with_condition<F>(mut self, condition: F) -> Self
    where
        F: Fn() -> bool + Send + Sync + 'static,
    {
        self.condition = Some(Box::new(condition));
        self
    }

    pub fn with_metrics(mut self, enable: bool) -> Self {
        self.enable_metrics = enable;
        self
    }
}

impl<T> Plugin for HotReloadPlugin<T>
where
    T: Asset + TypePath + DeserializeOwned + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        // Check if reload is enabled
        if let Some(condition) = &self.condition {
            if !condition() {
                info!("Hot-reload disabled for {}", std::any::type_name::<T>());
                return;
            }
        }

        // Register asset loader
        app.init_asset::<T>()
            .register_asset_loader(RonAssetLoader::<T>::default());

        // Set up verbose logging
        app.insert_resource(VerboseLogging(self.verbose));

        // Initial load of asset
        let asset_server = app.world().resource::<AssetServer>();
        let handle: Handle<T> = asset_server.load(&self.path);
        app.insert_resource(HotAssetHandle(handle));

        // Register events
        app.add_message::<AssetReloaded<T>>();

        // Add metrics if enabled
        if self.enable_metrics {
            app.insert_resource(ReloadMetrics::<T>::new(50));
        }

        // Register systems - only handle_asset_changes, Bevy's asset system handles file watching
        app.add_systems(Update, handle_asset_changes::<T>);

        info!(
            "🔥 Hot-reload enabled for {}: {}",
            std::any::type_name::<T>(),
            self.path
        );
    }
}
