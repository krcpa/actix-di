use std::any::{Any, TypeId};
use async_trait::async_trait;
use crate::error::ServiceError;

#[async_trait]
pub trait Service: Any + Send + Sync {
    async fn init(&self) -> Result<(), ServiceError> {
        Ok(())
    }

    async fn shutdown(&self) -> Result<(), ServiceError> {
        Ok(())
    }
}

pub trait DependencyProvider: Service {
    fn required_services() -> Vec<TypeId> where Self: Sized;
}