use std::{
    error::Error,
    fmt,
    sync::{Arc, RwLock, RwLockReadGuard},
};

#[derive(Debug)]
struct ReadOnlyError;

impl fmt::Display for ReadOnlyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not read ReadOnly variable")
    }
}

impl Error for ReadOnlyError {}

/** Read only */
#[derive(Debug, Clone)]
pub struct RO<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> RO<T> {
    pub fn new(val: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(val)),
        }
    }

    pub fn read(&self) -> Result<RwLockReadGuard<'_, T>, Box<dyn std::error::Error>> {
        match self.inner.read() {
            Ok(r) => Ok(r),
            Err(_) => Err(Box::new(ReadOnlyError)),
        }
    }
}
