use std::sync::Arc;

use crate::{
    service::{Service, ServiceProvider},
    service_lifetime::ServiceLifetime,
};

#[derive(Clone)]
pub struct ServiceDefinition {
    pub lifetime: ServiceLifetime,
    pub init: std::sync::Arc<dyn Fn(&dyn ServiceProvider) -> Arc<dyn Service> + Send + Sync>,
}
