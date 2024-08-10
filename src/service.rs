use std::{any::Any, sync::Arc};

use crate::service_provider::ServiceProvider;

/// Service trait
pub trait Service: Send + Sync + 'static {
    fn init(service_provider: &ServiceProvider) -> Arc<dyn Service>
    where
        Self: Sized;

    fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
}

pub struct ServiceResolver<T: ?Sized + 'static> {
    pub as_interface: fn(service: Arc<dyn Any + Sync + Send + 'static>) -> Arc<T>,
}
