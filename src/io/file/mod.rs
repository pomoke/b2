pub mod efi;

pub struct File<T> {
    backend: T,
}

impl<T> File<T> {
    /// To close backend, implement Drop for backend, or use scoped file struct in backend.
    pub fn close(self) {}
}
