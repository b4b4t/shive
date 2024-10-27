#[cfg(feature = "derive")]
mod derive_service_tests {
    use shive::service::Service;
    use shive::service::ServiceProvider;
    use shive::service::{get_trait_instance, ServiceResolver};
    use shive::{service::get_instance, service_container::ServiceContainer};
    use shive_derive::Service;
    use std::sync::Arc;

    pub trait TestTrait: Sync + Send {
        fn is_trait_ok(&self) -> bool;
    }

    #[derive(Service)]
    pub struct ServiceTestType {}

    impl ServiceTestType {
        pub fn is_ok(&self) -> bool {
            true
        }
    }

    impl TestTrait for ServiceTestType {
        fn is_trait_ok(&self) -> bool {
            true
        }
    }

    #[derive(Service)]
    pub struct CallerServiceTestDerive {
        test_derive: Arc<ServiceTestType>,
        test_trait_derive: Arc<dyn TestTrait>,
    }

    impl CallerServiceTestDerive {
        pub fn is_ok(&self) -> bool {
            self.test_derive.is_ok()
        }
        pub fn is_trait_ok(&self) -> bool {
            self.test_trait_derive.is_trait_ok()
        }
    }

    #[test]
    fn get_derive_instance_singleton_ok() {
        let mut service_container = ServiceContainer::new();
        let service_resolver = ServiceResolver::<dyn TestTrait> {
            as_interface: |resolver| resolver.downcast::<ServiceTestType>().unwrap(),
        };
        service_container.add_trait_singleton::<dyn TestTrait, ServiceTestType>(service_resolver);

        let service_provider = service_container.build();
        let scope = service_provider.create_scope();
        let service = get_trait_instance::<dyn TestTrait>(&scope).expect("Cannot get service");

        assert_eq!(service.is_trait_ok(), true);
    }

    #[test]
    fn get_derive_instance_scoped_from_singleton_ok() {
        let mut service_container = ServiceContainer::new();
        service_container.add_scoped::<CallerServiceTestDerive>();
        let service_resolver = ServiceResolver::<dyn TestTrait> {
            as_interface: |resolver| resolver.downcast::<ServiceTestType>().unwrap(),
        };
        service_container.add_trait_singleton::<dyn TestTrait, ServiceTestType>(service_resolver);
        service_container.add_singleton::<ServiceTestType>();

        let service_provider = service_container.build();
        let scope = service_provider.create_scope();
        let service = get_instance::<CallerServiceTestDerive>(&scope).expect("Cannot get service");

        assert_eq!(service.is_ok(), true);
    }

    #[test]
    fn get_derive_instance_transient_from_singleton_ok() {
        let mut service_container = ServiceContainer::new();
        service_container.add_transient::<CallerServiceTestDerive>();
        let service_resolver = ServiceResolver::<dyn TestTrait> {
            as_interface: |resolver| resolver.downcast::<ServiceTestType>().unwrap(),
        };
        service_container.add_trait_singleton::<dyn TestTrait, ServiceTestType>(service_resolver);
        service_container.add_singleton::<ServiceTestType>();
        let service_provider = service_container.build();
        let scope = service_provider.create_scope();
        let service = get_instance::<CallerServiceTestDerive>(&scope).expect("Cannot get service");

        assert_eq!(service.is_ok(), true);
    }

    #[test]
    fn get_derive_trait_instance_singleton_from_singleton_ok() {
        let mut service_container = ServiceContainer::new();
        service_container.add_singleton::<CallerServiceTestDerive>();
        let service_resolver = ServiceResolver::<dyn TestTrait> {
            as_interface: |resolver| resolver.downcast::<ServiceTestType>().unwrap(),
        };
        service_container.add_trait_singleton::<dyn TestTrait, ServiceTestType>(service_resolver);
        service_container.add_singleton::<ServiceTestType>();
        let service_provider = service_container.build();
        let scope = service_provider.create_scope();
        let service = get_instance::<CallerServiceTestDerive>(&scope).expect("Cannot get service");

        assert_eq!(service.is_trait_ok(), true);
    }

    #[test]
    fn get_derive_instance_scoped_ok() {
        let mut service_container = ServiceContainer::new();
        let service_resolver = ServiceResolver::<dyn TestTrait> {
            as_interface: |resolver| resolver.downcast::<ServiceTestType>().unwrap(),
        };
        service_container.add_trait_scoped::<dyn TestTrait, ServiceTestType>(service_resolver);

        let service_provider = service_container.build();
        let scope = service_provider.create_scope();
        let service = get_trait_instance::<dyn TestTrait>(&scope).expect("Cannot get service");

        assert_eq!(service.is_trait_ok(), true);
    }

    #[test]
    fn get_derive_instance_scoped_from_scoped_ok() {
        let mut service_container = ServiceContainer::new();
        service_container.add_scoped::<CallerServiceTestDerive>();
        let service_resolver = ServiceResolver::<dyn TestTrait> {
            as_interface: |resolver| resolver.downcast::<ServiceTestType>().unwrap(),
        };
        service_container.add_trait_scoped::<dyn TestTrait, ServiceTestType>(service_resolver);
        service_container.add_singleton::<ServiceTestType>();

        let service_provider = service_container.build();
        let scope = service_provider.create_scope();
        let service = get_instance::<CallerServiceTestDerive>(&scope).expect("Cannot get service");

        assert_eq!(service.is_ok(), true);
    }

    #[test]
    fn get_derive_instance_transient_from_scoped_ok() {
        let mut service_container = ServiceContainer::new();
        service_container.add_transient::<CallerServiceTestDerive>();
        let service_resolver = ServiceResolver::<dyn TestTrait> {
            as_interface: |resolver| resolver.downcast::<ServiceTestType>().unwrap(),
        };
        service_container.add_trait_scoped::<dyn TestTrait, ServiceTestType>(service_resolver);
        service_container.add_singleton::<ServiceTestType>();
        let service_provider = service_container.build();
        let scope = service_provider.create_scope();
        let service = get_instance::<CallerServiceTestDerive>(&scope).expect("Cannot get service");

        assert_eq!(service.is_ok(), true);
    }

    #[test]
    #[should_panic]
    fn get_derive_trait_instance_singleton_from_scoped_should_panic() {
        let mut service_container = ServiceContainer::new();
        service_container.add_singleton::<CallerServiceTestDerive>();
        let service_resolver = ServiceResolver::<dyn TestTrait> {
            as_interface: |resolver| resolver.downcast::<ServiceTestType>().unwrap(),
        };
        service_container.add_trait_scoped::<dyn TestTrait, ServiceTestType>(service_resolver);
        service_container.add_singleton::<ServiceTestType>();
        let service_provider = service_container.build();
        let scope = service_provider.create_scope();
        let _ = get_instance::<CallerServiceTestDerive>(&scope).expect("Cannot get service");
    }

    #[test]
    fn get_derive_instance_transient_ok() {
        let mut service_container = ServiceContainer::new();
        let service_resolver = ServiceResolver::<dyn TestTrait> {
            as_interface: |resolver| resolver.downcast::<ServiceTestType>().unwrap(),
        };
        service_container.add_trait_transient::<dyn TestTrait, ServiceTestType>(service_resolver);

        let service_provider = service_container.build();
        let scope = service_provider.create_scope();
        let service = get_trait_instance::<dyn TestTrait>(&scope).expect("Cannot get service");

        assert_eq!(service.is_trait_ok(), true);
    }

    #[test]
    fn get_derive_instance_scoped_from_transient_ok() {
        let mut service_container = ServiceContainer::new();
        service_container.add_scoped::<CallerServiceTestDerive>();
        let service_resolver = ServiceResolver::<dyn TestTrait> {
            as_interface: |resolver| resolver.downcast::<ServiceTestType>().unwrap(),
        };
        service_container.add_trait_transient::<dyn TestTrait, ServiceTestType>(service_resolver);
        service_container.add_scoped::<ServiceTestType>();

        let service_provider = service_container.build();
        let scope = service_provider.create_scope();
        let service = get_instance::<CallerServiceTestDerive>(&scope).expect("Cannot get service");

        assert_eq!(service.is_ok(), true);
    }

    #[test]
    fn get_derive_instance_transient_from_transient_ok() {
        let mut service_container = ServiceContainer::new();
        service_container.add_transient::<CallerServiceTestDerive>();
        let service_resolver = ServiceResolver::<dyn TestTrait> {
            as_interface: |resolver| resolver.downcast::<ServiceTestType>().unwrap(),
        };
        service_container.add_trait_transient::<dyn TestTrait, ServiceTestType>(service_resolver);
        service_container.add_scoped::<ServiceTestType>();
        let service_provider = service_container.build();
        let scope = service_provider.create_scope();
        let service = get_instance::<CallerServiceTestDerive>(&scope).expect("Cannot get service");

        assert_eq!(service.is_ok(), true);
    }

    #[test]
    #[should_panic]
    fn get_derive_trait_instance_singleton_from_transient_should_panic() {
        let mut service_container = ServiceContainer::new();
        service_container.add_singleton::<CallerServiceTestDerive>();
        let service_resolver = ServiceResolver::<dyn TestTrait> {
            as_interface: |resolver| resolver.downcast::<ServiceTestType>().unwrap(),
        };
        service_container.add_trait_transient::<dyn TestTrait, ServiceTestType>(service_resolver);
        service_container.add_scoped::<ServiceTestType>();
        let service_provider = service_container.build();
        let scope = service_provider.create_scope();
        let _ = get_instance::<CallerServiceTestDerive>(&scope).expect("Cannot get service");
    }
}
