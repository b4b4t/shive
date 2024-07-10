use std::sync::Arc;

use crate::{
    service::Service, service_lifetime::ServiceLifetime, service_provider::ServiceProvider,
};

#[derive(Clone)]
pub struct ServiceDefinition {
    pub lifetime: ServiceLifetime,
    pub init: std::sync::Arc<dyn Fn(&ServiceProvider) -> Arc<dyn Service> + Send + Sync>,
}
