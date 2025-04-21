#[cfg(feature = "sync")]
mod internal {
  use atomic::Atomic;
  use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

  #[derive(Debug, Default)]
  pub struct MaybeRefCell<T> {
    inner: RwLock<T>,
  }

  impl<T> MaybeRefCell<T> {
    pub fn new(value: T) -> Self {
      Self {
        inner: RwLock::new(value),
      }
    }

    pub fn borrow(&self) -> RwLockReadGuard<'_, T> {
      self.inner.read().unwrap()
    }

    pub fn borrow_mut(&self) -> RwLockWriteGuard<'_, T> {
      self.inner.write().unwrap()
    }
  }

  pub use std::sync::Arc as MaybeArc;

  #[derive(Debug, Default)]
  pub struct MaybeCell<T: bytemuck::Pod> {
    inner: Atomic<T>,
  }

  impl<T: bytemuck::Pod + Copy> MaybeCell<T> {
    pub fn new(value: T) -> Self {
      Self {
        inner: Atomic::new(value),
      }
    }

    pub fn get(&self) -> T {
      self.inner.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn set(&self, value: T) {
      self.inner.store(value, std::sync::atomic::Ordering::SeqCst);
    }
  }
}

#[cfg(not(feature = "sync"))]
mod internal {
  pub use std::cell::Cell as MaybeCell;
  pub use std::cell::RefCell as MaybeRefCell;
  pub use std::rc::Rc as MaybeArc;
}

pub use internal::{MaybeArc, MaybeCell, MaybeRefCell};
