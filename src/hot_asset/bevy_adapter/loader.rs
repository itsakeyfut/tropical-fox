use bevy::asset::{Asset, AssetLoader, LoadContext};
use bevy::reflect::TypePath;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RonLoaderError {
    #[error("Failed to read asset: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse RON: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

pub struct RonAssetLoader<T> {
    _phantom: PhantomData<T>,
}

impl<T> Default for RonAssetLoader<T> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> AssetLoader for RonAssetLoader<T>
where
    T: Asset + DeserializeOwned + TypePath,
{
    type Asset = T;
    type Settings = ();
    type Error = RonLoaderError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let asset = ron::de::from_bytes::<T>(&bytes)?;

        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
