#[derive(Copy, Clone)]
pub enum ServiceLifetime {
    Singleton,
    Scoped,
    Unmanaged,
    Transient,
}
