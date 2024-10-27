#[macro_export]
macro_rules! create_resolver {
    ($trait_type:ty, $concrete_type:ty) => {
        ServiceResolver::<$trait_type> {
            as_interface: |resolver| resolver.downcast::<$concrete_type>().unwrap(),
        }
    };
}
