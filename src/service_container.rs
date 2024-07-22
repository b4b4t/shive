use std::{collections::HashMap, sync::Arc};

use crate::{
    root_service_provider::RootServiceProvider, service::Service,
    service_definition::ServiceDefinition, service_lifetime::ServiceLifetime,
};

#[derive(Clone)]
pub struct ServiceContainer {
    service_collection: HashMap<String, ServiceDefinition>,
    pub unmanaged_services: HashMap<String, Arc<dyn Service>>,
    pub trait_service_map: HashMap<String, String>,
}

impl ServiceContainer {
    pub fn new() -> Self {
        Self {
            service_collection: HashMap::new(),
            unmanaged_services: HashMap::new(),
            trait_service_map: HashMap::new(),
        }
    }

    /// Declare and create a singleton in the service container.
    pub fn add_singleton<T: Service + 'static>(&mut self) {
        self.add_service::<T>(ServiceLifetime::Singleton, None);
    }

    /// Declare and create a singleton in the service container.
    pub fn add_trait_singleton<I: ?Sized, T: Service + 'static>(&mut self) {
        self.add_trait_service::<I, T>(ServiceLifetime::Singleton, None);
    }

    /// Declare and create a scoped instance in the service container.
    pub fn add_scoped<T: Service + 'static>(&mut self) {
        self.add_service::<T>(ServiceLifetime::Scoped, None);
    }

    /// Declare and create a transient instance in the service container.
    pub fn add_transient<T: Service + 'static>(&mut self) {
        self.add_service::<T>(ServiceLifetime::Transient, None);
    }

    /// Declare and create an unmanaged instance in the service container.
    pub fn add_unmanaged<T: Service + 'static>(&mut self, instance: T) {
        self.add_service::<T>(ServiceLifetime::Unmanaged, Some(instance));
    }

    /// Add a trait service with its lifetime and instance
    fn add_trait_service<I: ?Sized, T: Service + 'static>(
        &mut self,
        lifetime: ServiceLifetime,
        instance: Option<T>,
    ) {
        let trait_name = std::any::type_name::<I>().to_string();
        let service_name = std::any::type_name::<T>().to_string();

        println!("Add : {} -> {}", trait_name, service_name);

        // Associate the trait with the service
        self.trait_service_map.insert(trait_name, service_name);

        // Add the service
        self.add_service::<T>(lifetime, instance);
    }

    /// Add a service with its lifetime and instance
    fn add_service<T: Service + 'static>(
        &mut self,
        lifetime: ServiceLifetime,
        instance: Option<T>,
    ) {
        let service_init = T::init;
        let type_name = std::any::type_name::<T>().to_string();

        let service_definition = ServiceDefinition {
            init: Arc::new(service_init),
            lifetime,
        };

        self.service_collection
            .insert(type_name.to_string(), service_definition);

        if matches!(lifetime, ServiceLifetime::Unmanaged) && instance.is_some() {
            self.unmanaged_services
                .insert(type_name, Arc::new(instance.unwrap()));
        }
    }

    /// Get ServiceInstance from the service container
    pub fn get_service_definition_from_key(&self, type_name: String) -> Option<&ServiceDefinition> {
        self.service_collection.get(&type_name)
    }

    /// Get a root service provider to get the singleton and unmanaged services
    pub fn build(&self) -> RootServiceProvider {
        RootServiceProvider::new(self)
    }
}
