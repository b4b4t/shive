use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    root_service_provider::RootServiceProvider,
    service::{Service, ServiceResolver},
    service_lifetime::ServiceLifetime,
};

use super::error::Error;

#[derive(Clone)]
pub struct ScopedServiceProvider<'a> {
    pub services: Arc<RwLock<HashMap<String, Arc<dyn Service>>>>,
    pub root: &'a RootServiceProvider<'a>,
}

impl<'a> ScopedServiceProvider<'a> {
    /// Create service manger from service collection.
    pub fn new(root: &'a RootServiceProvider) -> Self {
        ScopedServiceProvider {
            services: Arc::new(RwLock::new(HashMap::new())),
            root,
        }
    }

    /// Get an instance of the specified type.
    /// Initialize new object depending on the lifetime.
    pub fn get_instance<T: Send + Sync + Service + 'static>(&self) -> Result<Arc<T>, Error> {
        let type_name = std::any::type_name::<T>().to_string();
        let service = Self::get_or_create_instance(self, type_name);

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

    pub fn get_trait_instance<T: ?Sized + Send + Sync + 'static>(&self) -> Result<Arc<T>, Error> {
        let trait_name = std::any::type_name::<T>();
        let service_name = self
            .root
            .service_container
            .trait_service_map
            .get(trait_name);

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

    /// Get or create an instance
    pub fn get_or_create_instance(&self, type_name: String) -> Result<Arc<dyn Service>, Error> {
        // Get service definition
        let service_definition = self
            .root
            .service_container
            .get_service_definition_from_key(type_name.clone());

        if service_definition.is_none() {
            return Err(Error::Internal("Service definition not found".to_string()));
        }

        let service_definition = service_definition.unwrap();

        // If the service is a singleton or unmanaged,
        // get the service in the root provider
        if matches!(service_definition.lifetime, ServiceLifetime::Unmanaged)
            || matches!(service_definition.lifetime, ServiceLifetime::Singleton)
        {
            return self.root.get_or_create_instance(type_name);
        }

        // Create a new service instance
        let init = service_definition.init.clone();
        let service = init(self);

        // Add new instance for scoped and singleton
        match service_definition.lifetime {
            ServiceLifetime::Scoped => {
                self.services
                    .write()
                    .unwrap()
                    .insert(type_name, service.clone());
            }
            _ => {}
        }

        Ok(service)
    }
}
