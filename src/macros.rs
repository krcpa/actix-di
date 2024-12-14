#[macro_export]
macro_rules! inject_service {
    ($state:expr, $service:ty) => {
        $state.get::<$service>()
            .ok_or_else(|| $crate::error::ServiceError::ServiceNotFound(std::any::TypeId::of::<$service>()))
    };
}

#[macro_export]
macro_rules! provide_dependencies {
    ($($dep:ty),*) => {
        fn required_services() -> Vec<std::any::TypeId> {
            vec![
                $(std::any::TypeId::of::<$dep>()),*
            ]
        }
    };
}