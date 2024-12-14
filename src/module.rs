use std::sync::Arc;
use crate::{DependencyProvider, Service, ServiceError, ServiceRegistry};
use crate::service::As;

/// Trait for implementing modular dependency registration
pub trait Module: Send + Sync {
    /// Register all dependencies in this module
    fn register(&self, registry: &mut ServiceRegistry) -> Result<(), ServiceError>;

    /// Optional initialization for the module
    async fn init(&self) -> Result<(), ServiceError> {
        Ok(())
    }

    /// Optional cleanup for the module
    async fn shutdown(&self) -> Result<(), ServiceError> {
        Ok(())
    }
}

/// Helper struct for building complex modules
pub struct ModuleBuilder {
    modules: Vec<Box<dyn Module>>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self { modules: vec![] }
    }

    pub fn add_module<M: Module + 'static>(&mut self, module: M) {
        self.modules.push(Box::new(module));
    }

    pub fn build(self) -> CompositeModule {
        CompositeModule {
            modules: self.modules,
        }
    }
}

/// A module that combines multiple other modules
pub struct CompositeModule {
    modules: Vec<Box<dyn Module>>,
}

impl Module for CompositeModule {
    fn register(&self, registry: &mut ServiceRegistry) -> Result<(), ServiceError> {
        for module in &self.modules {
            module.register(registry)?;
        }
        Ok(())
    }

    async fn init(&self) -> Result<(), ServiceError> {
        for module in &self.modules {
            module.init().await?;
        }
        Ok(())
    }

    async fn shutdown(&self) -> Result<(), ServiceError> {
        for module in &self.modules {
            module.shutdown().await?;
        }
        Ok(())
    }
}

/// Generic service registration helper
pub trait ServiceRegistration<T: ?Sized> {
    fn register_service<S>(&mut self, service: Arc<S>) -> Result<(), ServiceError>
    where
        S: Service + DependencyProvider + As<T> + 'static;
}

impl<T: ?Sized> ServiceRegistration<T> for ServiceRegistry {
    fn register_service<S>(&mut self, service: Arc<S>) -> Result<(), ServiceError>
    where
        S: Service + DependencyProvider + As<T> + 'static,
    {
        self.register(service)
    }
}
