use std::{any::Any, sync::Arc};

use crate::{error::Error, service_container::ServiceContainer};

/// Service trait
pub trait Service: Send + Sync + 'static {
    fn init(service_provider: &dyn ServiceProvider) -> Arc<dyn Service>
    where
        Self: Sized;

    fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
}

pub struct ServiceResolver<T: ?Sized + 'static> {
    pub as_interface: fn(service: Arc<dyn Any + Sync + Send + 'static>) -> Arc<T>,
}

pub trait ServiceProvider<'a> {
    fn as_service_provider(&'a self) -> &'a dyn ServiceProvider<'a>;
    fn get_or_create_instance(&self, type_name: String) -> Result<Arc<dyn Service>, Error>;
    fn get_service_container(&self) -> &ServiceContainer;
}

/// Get an instance of the specified type.
/// Initialize new object depending on the lifetime.
pub fn get_instance<T: Send + Sync + Service + 'static>(
    service_provider: &dyn ServiceProvider,
) -> Result<Arc<T>, Error> {
    let type_name = std::any::type_name::<T>().to_string();
    let service = service_provider.get_or_create_instance(type_name);

    match service {
        Ok(srv) => {
            // Return the created service
            match Arc::downcast::<T>(srv.as_any()) {
                Ok(obj) => Ok(obj),
                Err(_) => Err(Error::Internal("Cannot downcast service".to_string())),
            }
        }
        Err(error) => Err(error),
    }
}

/// Get an instance of the specified trait.
/// Initialize new object depending on the lifetime.
pub fn get_trait_instance<T: ?Sized + Send + Sync + 'static>(
    service_provider: &dyn ServiceProvider,
) -> Result<Arc<T>, Error> {
    let trait_name = std::any::type_name::<T>();
    let service_name = service_provider
        .get_service_container()
        .trait_service_map
        .get(trait_name);

    return match service_name {
        Some(type_name) => {
            // Get or create service
            let service = service_provider.get_or_create_instance(trait_name.to_string());

            // Get service resolver
            let service_resolver = type_name
                .downcast_ref::<ServiceResolver<T>>()
                .expect("Cannot get service resolver");

            Ok((service_resolver.as_interface)(service.unwrap().as_any()))
        }
        None => Err(Error::Internal("Cannot downcast service".to_string())),
    };
}
