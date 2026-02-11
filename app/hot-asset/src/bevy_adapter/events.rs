use bevy::prelude::*;
use std::marker::PhantomData;
use std::time::SystemTime;

#[derive(Message)]
pub struct AssetReloaded<T: Asset> {
    pub handle: Handle<T>,
    pub success: bool,
    pub error: Option<String>,
    pub timestamp: SystemTime,
    _phantom: PhantomData<T>,
}

impl<T: Asset> AssetReloaded<T> {
    pub fn success(handle: Handle<T>) -> Self {
        Self {
            handle,
            success: true,
            error: None,
            timestamp: SystemTime::now(),
            _phantom: PhantomData,
        }
    }

    pub fn failure(handle: Handle<T>, error: String) -> Self {
        Self {
            handle,
            success: false,
            error: Some(error),
            timestamp: SystemTime::now(),
            _phantom: PhantomData,
        }
    }
}
