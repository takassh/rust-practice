use std::{
    cell::UnsafeCell,
    sync::atomic::{
        AtomicBool,
        Ordering::{Acquire, Release},
    }, thread,
};

// Guard's lifetime is shorter than lock's lifetime.
pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

use std::ops::{Deref, DerefMut};

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // Safety: The existence of this guard itself guarantees that
        // the lock has been exclusively acquired.
        unsafe { &*self.lock.value.get() }
    }
}
impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: The existence of this guard itself guarantees that
        // the lock has been exclusively acquired.
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
}

unsafe impl<T> Send for Guard<'_, T> where T: Send {} // this is not required because SpinLock is already send
unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }

        Guard { lock: self }
    }

    // This is not required because the lock is released when the guard is dropped.
    // pub fn unlock(&self) {
    //     self.locked.store(false, Release);
    // }
}

fn main() {
    let x = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| x.lock().push(1));
        s.spawn(|| {
            let mut g = x.lock();
            g.push(2);
            g.push(2);
        });
    });
    let g = x.lock();
    assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);
    println!("All tests passed!")
}
