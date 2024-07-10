use std::sync::Arc;

use shive::root_service_provider::RootServiceProvider;
use shive::service_provider::ServiceProvider;
use shive::{service::Service, service_container::ServiceContainer};

#[derive(Clone)]
pub struct TestType;

impl Service for TestType {
    fn init(_: &ServiceProvider) -> Arc<dyn Service>
    where
        Self: Sized,
    {
        Arc::new(Self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TestType {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_ok(&self) -> bool {
        true
    }
}

#[test]
fn get_instance_singleton_ok() {
    let mut service_container = ServiceContainer::new();
    service_container.add_singleton::<TestType>();

    let service_provider = RootServiceProvider::new(service_container);
    let service = service_provider
        .get_instance::<TestType>()
        .expect("Cannot get service");

    assert_eq!(service.is_ok(), true);
}

#[test]
fn get_instance_unmanaged_ok() {
    let mut service_container = ServiceContainer::new();
    service_container.add_unmanaged::<TestType>(TestType::new());

    let service_provider = RootServiceProvider::new(service_container);
    let service = service_provider
        .get_instance::<TestType>()
        .expect("Cannot get service");

    assert_eq!(service.is_ok(), true);
}

#[test]
fn get_instance_scoped_ok() {
    let mut service_container = ServiceContainer::new();
    service_container.add_scoped::<TestType>();
    let root_provider = RootServiceProvider::new(service_container);
    let service_provider = root_provider.create_scope();
    let service: Arc<TestType> = service_provider
        .get_instance::<TestType>()
        .expect("Cannot get service");

    assert_eq!(service.is_ok(), true);
}

#[test]
fn get_instance_singleton_not_found() {
    let service_container = ServiceContainer::new();
    let service_provider = RootServiceProvider::new(service_container);
    let service = service_provider.get_instance::<TestType>();

    assert_eq!(service.is_err(), true);
}
