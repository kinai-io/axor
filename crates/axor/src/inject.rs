use std::sync::{Arc, RwLock};

use crate::AxorContext;

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

    /// Framework usage only
    pub fn resolve_service(&self, ctx: &AxorContext)
    where
        T: Send + Sync + 'static,
    {
        let service = ctx
            .get_service::<T>()
            .expect(&format!("Missing service: {}", std::any::type_name::<T>()));
        self.inject(service);
    }
}
