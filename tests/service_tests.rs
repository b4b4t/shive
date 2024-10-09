use std::sync::Arc;

use shive::scoped_service_provider::ScopedServiceProvider;
use shive::service::ServiceResolver;
use shive::{service::Service, service_container::ServiceContainer};

pub trait TestTrait: Sync + Send + 'static {
    fn is_trait_ok(&self) -> bool;
}

#[derive(Clone)]
pub struct TestType;

impl Service for TestType {
    fn init(_: &ScopedServiceProvider) -> Arc<dyn Service>
    where
        Self: Sized,
    {
        Arc::new(Self)
    }

    fn as_any(self: Arc<Self>) -> Arc<dyn std::any::Any + Send + Sync> {
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

    // pub fn as_interface(&self) -> Arc<&dyn TestTrait> {
    //     Arc::new(self)
    // }
}

impl TestType {
    pub fn test() {}
}

impl TestTrait for TestType {
    fn is_trait_ok(&self) -> bool {
        true
    }
}

pub struct TestTypeCaller {
    test_type: Arc<TestType>,
}

impl Service for TestTypeCaller {
    fn init(service_provider: &ScopedServiceProvider) -> Arc<dyn Service>
    where
        Self: Sized,
    {
        Arc::new(Self {
            test_type: service_provider
                .get_instance::<TestType>()
                .expect("Cannot get TestType"),
        })
    }

    fn as_any(self: Arc<Self>) -> Arc<dyn std::any::Any + Send + Sync> {
        self
    }
}

impl TestTypeCaller {
    pub fn is_call_ok(&self) -> bool {
        self.test_type.is_ok()
    }
}

#[test]
fn get_instance_singleton_ok() {
    let mut service_container = ServiceContainer::new();
    service_container.add_singleton::<TestType>();
    let service_provider = service_container.build();
    let service = service_provider
        .get_instance::<TestType>()
        .expect("Cannot get service");

    assert_eq!(service.is_ok(), true);
}

#[test]
fn get_instance_unmanaged_ok() {
    let mut service_container = ServiceContainer::new();
    service_container.add_unmanaged::<TestType>(TestType::new());
    let service_provider = service_container.build();
    let service = service_provider
        .get_instance::<TestType>()
        .expect("Cannot get service");

    assert_eq!(service.is_ok(), true);
}

#[test]
fn get_instance_scoped_ok() {
    let mut service_container = ServiceContainer::new();
    service_container.add_scoped::<TestType>();
    let root_provider = service_container.build();
    let service_provider = root_provider.create_scope();
    let service: Arc<TestType> = service_provider
        .get_instance::<TestType>()
        .expect("Cannot get service");

    assert_eq!(service.is_ok(), true);
}

#[test]
fn get_instance_transient_ok() {
    let mut service_container = ServiceContainer::new();
    service_container.add_transient::<TestType>();
    let root_provider = service_container.build();
    let service_provider = root_provider.create_scope();
    let service: Arc<TestType> = service_provider
        .get_instance::<TestType>()
        .expect("Cannot get service");

    assert_eq!(service.is_ok(), true);
}

#[test]
fn get_instance_singleton_not_found() {
    let service_container = ServiceContainer::new();
    let service_provider = service_container.build();
    let service = service_provider.get_instance::<TestType>();

    assert_eq!(service.is_err(), true);
}

#[test]
fn get_instance_trait_singleton_ok() {
    let mut service_container = ServiceContainer::new();
    let service_resolver = ServiceResolver::<dyn TestTrait> {
        as_interface: |resolver| resolver.downcast::<TestType>().unwrap(),
    };
    service_container.add_trait_singleton::<dyn TestTrait, TestType>(service_resolver);
    let service_provider = service_container.build();
    let service = service_provider
        .get_trait_instance::<dyn TestTrait>()
        .expect("Cannot get service");

    assert_eq!(service.is_trait_ok(), true);
}

#[test]
fn get_instance_trait_unmanaged_ok() {
    let mut service_container = ServiceContainer::new();
    let service_resolver = ServiceResolver::<dyn TestTrait> {
        as_interface: |resolver| resolver.downcast::<TestType>().unwrap(),
    };
    service_container
        .add_trait_unmanaged::<dyn TestTrait, TestType>(service_resolver, TestType::new());
    let service_provider = service_container.build();
    let service = service_provider
        .get_trait_instance::<dyn TestTrait>()
        .expect("Cannot get service");

    assert_eq!(service.is_trait_ok(), true);
}

#[test]
fn get_instance_trait_scoped_ok() {
    let mut service_container = ServiceContainer::new();
    let service_resolver = ServiceResolver::<dyn TestTrait> {
        as_interface: |resolver| resolver.downcast::<TestType>().unwrap(),
    };
    service_container.add_trait_scoped::<dyn TestTrait, TestType>(service_resolver);
    let root_provider = service_container.build();
    let service_provider = root_provider.create_scope();
    let service = service_provider
        .get_trait_instance::<dyn TestTrait>()
        .expect("Cannot get service");

    assert_eq!(service.is_trait_ok(), true);
}

#[test]
fn get_instance_trait_transient_ok() {
    let mut service_container = ServiceContainer::new();
    let service_resolver = ServiceResolver::<dyn TestTrait> {
        as_interface: |resolver| resolver.downcast::<TestType>().unwrap(),
    };
    service_container.add_trait_transient::<dyn TestTrait, TestType>(service_resolver);
    let root_provider = service_container.build();
    let service_provider = root_provider.create_scope();
    let service = service_provider
        .get_trait_instance::<dyn TestTrait>()
        .expect("Cannot get service");

    assert_eq!(service.is_trait_ok(), true);
}

#[test]
fn get_instance_trait_scoped_from_scoped_ok() {
    let mut service_container = ServiceContainer::new();
    service_container.add_scoped::<TestType>();
    service_container.add_scoped::<TestTypeCaller>();
    let root_provider = service_container.build();
    let service_provider = root_provider.create_scope();
    let service = service_provider
        .get_instance::<TestTypeCaller>()
        .expect("Cannot get service");

    assert_eq!(service.is_call_ok(), true);
}

#[test]
fn get_instance_trait_scoped_from_singleton_ko() {
    let mut service_container = ServiceContainer::new();
    service_container.add_scoped::<TestType>();
    service_container.add_singleton::<TestTypeCaller>();
    let root_provider = service_container.build();
    let service_provider = root_provider.create_scope();
    let service = service_provider
        .get_instance::<TestTypeCaller>()
        .expect("Cannot get service");

    assert_eq!(service.is_call_ok(), false);
}

#[test]
fn get_instance_trait_scoped_from_transient_ok() {
    let mut service_container = ServiceContainer::new();
    service_container.add_scoped::<TestType>();
    service_container.add_transient::<TestTypeCaller>();
    let root_provider = service_container.build();
    let service_provider = root_provider.create_scope();
    let service = service_provider
        .get_instance::<TestTypeCaller>()
        .expect("Cannot get service");

    assert_eq!(service.is_call_ok(), true);
}
