# Shive - Services Hive

## :honeybee: Introduction

Shive, for service hive, is lightweight IOC service container writen for the Rust applications. 
This is a basic implementation of an IOC which manages the services from a container.

## :checkered_flag: Installation

To use the library in your project, you can add the following line in the Cargo.toml file :

`shive = { version = "0.1.0-alpha.1", features = ["derive"] }`

Work in progress and this is not production ready, please proceed with caution.

## :rocket: Get started

### Create a service

A service is a struct that implements the `Service` trait to be initialized by the service provider.
If the service depends on other services, these must be added in the struct properties and they *must be hosted in a Arc pointer*.

Example : 

```rust
#[derive(Clone)]
pub struct TestService {
    test_repository: Arc<TestRepository>,
}

impl Service for TestService {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn init(service_provider: &dyn ServiceProvider) -> Arc<dyn Service>
    where
        Self: Sized,
    {
        let test_repository = get_instance::<TestRepository>(service_provider)
            .expect("Error to retrieve instance");

        Arc::new(Self { test_repository })
    }
}
```

By using the `derive` feature, you can simplify this declaration with the `Service` derive macro :

```rust
#[derive(Service, Clone)]
pub struct TestService {
    test_repository: Arc<TestRepository>,
}
```

### Create a service container

To create a service container, use the `new` method.

Example :

```rust
let mut service_container = ServiceContainer::new();
```

### Declare a service

4 lifetimes that can be declared in the service container :

- singleton : services that have the same lifetime as the container.

```rust
service_container.add_singleton::<TestType>();
```

- scoped : services that live until the end of the service provider.

```rust
service_container.add_scoped::<TestType>();
```

- transient : services created for each call to the service provider.

```rust
service_container.add_transient::<TestType>();
```

- unmanaged : services that are not managed by the service provider. The service is provided manually when it is declared in the container.

```rust
service_container.add_unmanaged::<TestType>(TestType::new());
```

### Declare a service by using a trait

To be used as a service, a trait must be assigned to a stuct having its implementation and a resolver has to be created in order to downcast the service.

Example :

```rust
let service_resolver = ServiceResolver::<dyn TestTrait> {
    as_interface: |resolver| resolver.downcast::<TestType>().unwrap(),
};
```

To declare a service by using a trait, there are equivalent methods to the service declaration :

- singleton : services that have the same lifetime as the container.

```rust
service_container.add_trait_singleton::<dyn TestTrait, TestType>(service_resolver);
```
- scoped : services that live until the end of the service provider.

```rust
service_container.add_trait_scoped::<dyn TestTrait, TestType>(service_resolver);
```

- transient : services created for each call to the service provider.

```rust
service_container.add_trait_transient::<dyn TestTrait, TestType>(service_resolver);

```
- unmanaged : services that are not managed by the service provider. The service is provided manually when it is declared in the container.

```rust
service_container.add_trait_unmanaged::<dyn TestTrait, TestType>(service_resolver);
```

### Get a service provider

Service providers contain services that are scoped by its lifetime, singletons and unmanaged services. 

For the transient services, they are created each time they are requested from the service provider.

To acquire a service provider from a container, you need to build the service container to get the root service provider. Then, you can get the service provider by creating a new scope from this one.

Example :

```rust
let root_service_provider = service_container.build();
let service_provider = root_provider.create_scope();
```

### Get a service

A service can be get from a service provider with the `get_instance` method.

Example :

``` rust
let service = get_instance::<TestType>();
```

### Get a service by using a trait

A service can be get from a service provider with the `get_trait_instance` method.

Example :

``` rust
let service =
    get_trait_instance::<dyn TestTrait>(&service_provider).expect("Cannot get service");
```