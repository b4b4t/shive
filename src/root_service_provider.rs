use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    scoped_service_provider::ScopedServiceProvider,
    service::{Service, ServiceResolver},
    service_lifetime::ServiceLifetime,
};

use super::{error::Error, service_container::ServiceContainer};

/// Service provider providing singleton and unmanaged services
#[derive(Clone)]
pub struct RootServiceProvider<'a> {
    pub service_container: &'a ServiceContainer,
    pub singleton_services: Arc<RwLock<HashMap<String, Arc<dyn Service>>>>,
}

impl<'a> RootServiceProvider<'a> {
    /// Create service manger from service collection.
    pub fn new(sc: &'a ServiceContainer) -> Self {
        Self {
            service_container: sc,
            singleton_services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create service manger from service collection.
    pub fn create_scope(&self) -> ScopedServiceProvider {
        ScopedServiceProvider::new(self)
    }

    pub fn get_trait_instance<T: ?Sized + Send + Sync + 'static>(&self) -> Result<Arc<T>, Error> {
        let trait_name = std::any::type_name::<T>();
        let service_name = self.service_container.trait_service_map.get(trait_name);

        return match service_name {
            Some(type_name) => {
                // Get or create service
                let service = Self::get_or_create_instance(self, trait_name.to_string());

                // Get service resolver
                let service_resolver = type_name
                    .downcast_ref::<ServiceResolver<T>>()
                    .expect("Cannot get service resolver");

                Ok((service_resolver.as_interface)(service.unwrap().as_any()))
            }
            None => Err(Error::Internal("Cannot downcast service".to_string())),
        };
    }

    /// Get an instance of the specified type.
    /// Initialize new object depending on the lifetime.
    pub fn get_instance<T: Service + Send + Sync + 'static>(&self) -> Result<Arc<T>, Error> {
        let type_name = std::any::type_name::<T>().to_string();
        let service = Self::get_or_create_instance(self, type_name);

        match service {
            Ok(srv) =>
            // Return the created service
            {
                match Arc::downcast::<T>(srv.as_any()) {
                    Ok(obj) => Ok(obj),
                    Err(_) => Err(Error::Internal("Cannot downcast service".to_string())),
                }
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_or_create_instance(&self, type_name: String) -> Result<Arc<dyn Service>, Error> {
        // Get service definition
        let service_definition = self
            .service_container
            .get_service_definition_from_key(type_name.clone());

        if service_definition.is_none() {
            return Err(Error::Internal("Service definition not found".to_string()));
        }

        let service_definition = service_definition.unwrap();

        // If the service instance exists, return it
        // Search in unmanaged services
        if matches!(service_definition.lifetime, ServiceLifetime::Unmanaged) {
            let unmanaged_services = &self.service_container.unmanaged_services;
            if unmanaged_services.contains_key(&type_name) {
                let srv = unmanaged_services.get(&type_name);

                return match srv {
                    Some(service) => Ok(service.clone()),
                    None => Err(Error::Internal(format!(
                        "Cannot get the service instance for {}",
                        type_name
                    ))),
                };
            } else {
                return Err(Error::Internal(format!(
                    "Unmanaged instance for {} is not set",
                    type_name,
                )));
            }
        }

        // Search in singleton
        if matches!(service_definition.lifetime, ServiceLifetime::Singleton) {
            let singleton_services = self.singleton_services.read().unwrap();

            if singleton_services.contains_key(&type_name) {
                let srv = singleton_services.get(&type_name);

                return match srv {
                    Some(service) => Ok(service.clone()),
                    None => Err(Error::Internal(format!(
                        "Cannot get the service instance for {}",
                        type_name
                    ))),
                };
            }

            // Unlock the singleton services
            drop(singleton_services);

            // Create a new service instance
            let init = service_definition.init.clone();
            let service = init(&ScopedServiceProvider::new(self));

            // Add new instance for scoped and singleton
            self.singleton_services
                .write()
                .expect("blocked")
                .insert(type_name, service.clone());

            return Ok(service.clone());
        } else {
            // Scoped or transient services are not supported in root service provider
            // because it needs a scope. Instead, get the service from a service provider.
            return Err(Error::Internal(format!(
                "Cannot get the service instance for {} (scoped or transient services are not supported in root service provider)",
                type_name
            )));
        }
    }
}
