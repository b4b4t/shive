use std::{any::Any, collections::HashMap, sync::Arc};

use crate::{
    root_service_provider::RootServiceProvider,
    service::{Service, ServiceResolver},
    service_definition::ServiceDefinition,
    service_lifetime::ServiceLifetime,
};

pub struct ServiceContainer {
    service_collection: HashMap<String, ServiceDefinition>,
    pub unmanaged_services: HashMap<String, Arc<dyn Service>>,
    pub trait_service_map: HashMap<String, Arc<dyn Any + Send + Sync + 'static>>,
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
    pub fn add_trait_singleton<I: ?Sized + Send + Sync + 'static, T: Service + 'static>(
        &mut self,
        resolver: ServiceResolver<I>,
    ) {
        self.add_trait_service::<I, T>(ServiceLifetime::Singleton, None, resolver);
    }

    /// Declare and create a scoped instance in the service container.
    pub fn add_scoped<T: Service + 'static>(&mut self) {
        self.add_service::<T>(ServiceLifetime::Scoped, None);
    }

    /// Declare and create a scoped in the service container.
    pub fn add_trait_scoped<I: ?Sized + Send + Sync + 'static, T: Service + 'static>(
        &mut self,
        resolver: ServiceResolver<I>,
    ) {
        self.add_trait_service::<I, T>(ServiceLifetime::Scoped, None, resolver);
    }

    /// Declare and create a transient instance in the service container.
    pub fn add_transient<T: Service + 'static>(&mut self) {
        self.add_service::<T>(ServiceLifetime::Transient, None);
    }

    /// Declare and create a transient in the service container.
    pub fn add_trait_transient<I: ?Sized + Send + Sync + 'static, T: Service + 'static>(
        &mut self,
        resolver: ServiceResolver<I>,
    ) {
        self.add_trait_service::<I, T>(ServiceLifetime::Transient, None, resolver);
    }

    /// Declare and create an unmanaged instance in the service container.
    pub fn add_unmanaged<T: Service + 'static>(&mut self, instance: T) {
        self.add_service::<T>(ServiceLifetime::Unmanaged, Some(instance));
    }

    /// Declare and create an unmanaged instance in the service container.
    pub fn add_trait_unmanaged<I: ?Sized + Send + Sync + 'static, T: Service + 'static>(
        &mut self,
        resolver: ServiceResolver<I>,
        instance: T,
    ) {
        self.add_trait_service::<I, T>(ServiceLifetime::Unmanaged, Some(instance), resolver);
    }

    /// Add a trait service with its lifetime and instance
    fn add_trait_service<I: ?Sized + Send + Sync + 'static, T: Service + 'static>(
        &mut self,
        lifetime: ServiceLifetime,
        instance: Option<T>,
        resolver: ServiceResolver<I>,
    ) {
        let trait_name = std::any::type_name::<I>().to_string();
        let service_init: Arc<
            fn(&crate::scoped_service_provider::ScopedServiceProvider) -> Arc<dyn Service>,
        > = Arc::new(T::init);
        let service_instance: Option<Arc<dyn Service>> = match instance {
            Some(service) => Some(Arc::new(service)),
            None => None,
        };

        // Add the service
        self.add_keyed_service(&trait_name, lifetime, service_init, service_instance);

        // Add the resolver
        self.trait_service_map
            .insert(trait_name, Arc::new(resolver));
    }

    /// Add a service with its lifetime and instance
    fn add_service<T: Service + 'static>(
        &mut self,
        lifetime: ServiceLifetime,
        instance: Option<T>,
    ) {
        let service_init: Arc<
            fn(&crate::scoped_service_provider::ScopedServiceProvider) -> Arc<dyn Service>,
        > = Arc::new(T::init);
        let type_name = std::any::type_name::<T>().to_string();
        let service_instance: Option<Arc<dyn Service>> = match instance {
            Some(service) => Some(Arc::new(service)),
            None => None,
        };

        self.add_keyed_service(&type_name, lifetime, service_init, service_instance);
    }

    /// Add a service with its lifetime and instance
    fn add_keyed_service(
        &mut self,
        key: &str,
        lifetime: ServiceLifetime,
        init: Arc<fn(&crate::scoped_service_provider::ScopedServiceProvider) -> Arc<dyn Service>>,
        instance: Option<Arc<dyn Service>>,
    ) {
        let service_definition = ServiceDefinition { init, lifetime };

        self.service_collection
            .insert(key.to_string(), service_definition);

        if matches!(lifetime, ServiceLifetime::Unmanaged) && instance.is_some() {
            self.unmanaged_services
                .insert(key.to_string(), instance.unwrap());
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
