use core::cell::UnsafeCell;
use core::ops::Deref;
use core::ops::DerefMut;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

pub struct Mutex<T> {
    pub inner: UnsafeCell<T>,
    status: AtomicBool,
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
    pub fn new(data: T) -> Mutex<T> {
        Mutex {
            inner: UnsafeCell::new(data),
            status: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) -> Result<MutexGuard<T>, ()> {
        loop {
            match self
                .status
                .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            {
                Ok(_) => break,
                Err(_) => continue,
            }
        }

        Ok(MutexGuard { mutex: self })
    }
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.inner.get() }
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.inner.get() }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.status.store(false, Ordering::Relaxed);
    }
}
