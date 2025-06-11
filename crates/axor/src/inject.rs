use std::sync::{Arc, RwLock};

pub struct Inject<T> {
    inner: RwLock<Option<Arc<T>>>,
}

impl<T> Default for Inject<T> {
    fn default() -> Self {
        Self {
            inner: RwLock::new(None),
        }
    }
}

impl<T> Inject<T> {
    
    pub fn resolve(&self) -> Arc<T> {
        self.inner
            .read()
            .unwrap()
            .as_ref()
            .expect("Dependency not injected")
            .clone()
    }

    /// Framework usage only
    pub fn inject(&self, value: Arc<T>) {
        *self.inner.write().unwrap() = Some(value);
    }
}
