use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    root_service_provider::RootServiceProvider,
    service::{Service, ServiceProvider},
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
}

impl<'a> ServiceProvider<'a> for ScopedServiceProvider<'a> {
    /// Get or create an instance
    fn get_or_create_instance(&self, type_name: String) -> Result<Arc<dyn Service>, Error> {
        // Get service definition
        let service_definition = self
            .get_service_container()
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
        let service = init(self.as_service_provider());

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

    fn as_service_provider(&'a self) -> &'a dyn ServiceProvider<'a> {
        self
    }

    fn get_service_container(&self) -> &crate::service_container::ServiceContainer {
        self.root.service_container
    }
}
