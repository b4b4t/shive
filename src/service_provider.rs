use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    root_service_provider::RootServiceProvider, service::Service, service_lifetime::ServiceLifetime,
};

use super::{error::Error, service_container::ServiceContainer};

#[derive(Clone)]
pub struct ServiceProvider<'a> {
    pub service_container: ServiceContainer,
    pub services: Arc<RwLock<HashMap<String, Arc<dyn Service>>>>,
    pub root: &'a RootServiceProvider<'a>,
}

impl<'a> ServiceProvider<'a> {
    /// Create service manger from service collection.
    pub fn new(root: &'a RootServiceProvider) -> Self {
        ServiceProvider {
            service_container: root.service_container.clone(),
            services: Arc::new(RwLock::new(HashMap::new())),
            root,
        }
    }

    /// Get an instance of the specified type.
    /// Initialize new object depending on the lifetime.
    pub fn get_instance<T: Send + Sync + Service + Clone + 'static>(
        &self,
    ) -> Result<Arc<T>, Error> {
        let type_name = std::any::type_name::<T>().to_string();
        let service = Self::get_or_create_instance(self, type_name);

        match service {
            Ok(srv) => {
                // Return the created service
                match srv.as_any().downcast_ref::<T>() {
                    Some(obj) => Ok(Arc::new(obj.clone())),
                    None => Err(Error::Internal("Cannot downcast service".to_string())),
                }
            }
            Err(error) => Err(error),
        }
    }

    /// Get or create an instance
    pub fn get_or_create_instance(&self, type_name: String) -> Result<Arc<dyn Service>, Error> {
        // Get service definition
        let service_definition = self
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
