use std::sync::Arc;

use crate::{
    scoped_service_provider::ScopedServiceProvider, service::Service,
    service_lifetime::ServiceLifetime,
};

#[derive(Clone)]
pub struct ServiceDefinition {
    pub lifetime: ServiceLifetime,
    pub init: std::sync::Arc<dyn Fn(&ScopedServiceProvider) -> Arc<dyn Service> + Send + Sync>,
}
