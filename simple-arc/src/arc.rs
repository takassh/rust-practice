use std::cell::UnsafeCell;
use std::process::abort;
use std::sync::atomic::fence;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::{ops::Deref, ptr::NonNull, sync::atomic::AtomicUsize};

struct ArcData<T> {
    data_ref_count: AtomicUsize,
    alloc_ref_count: AtomicUsize,
    data: UnsafeCell<Option<T>>,
}

pub struct Arc<T> {
    weak: Weak<T>,
}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}

impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe { &self.ptr.as_ref() }
    }

    pub fn upgrade(&self) -> Option<Arc<T>> {
        let mut n = self.data().data_ref_count.load(Relaxed);
        loop {
            if n == 0 {
                return None;
            }
            assert!(n <= usize::MAX);
            if let Err(e) = self
                .data()
                .data_ref_count
                .compare_exchange(n, n + 1, Relaxed, Relaxed)
            {
                n = e;
                continue;
            }
            return Some(Arc { weak: self.clone() });
        }
    }
}

unsafe impl<T: Send + Sync> Send for Weak<T> {} // Arc can be sent between threads in this case, T must be Send and Sync
unsafe impl<T: Send + Sync> Sync for Weak<T> {} // Arc can be referenced between threads in this case, T must be Send and Sync

impl<T> Arc<T> {
    pub fn new(data: T) -> Self {
        // Box::leak is used to throw away the exclusive ownership to the Box, but keep the memory allocated on the heap.
        // So dropping the returned reference will cause a memory leak.
        let ptr = Box::leak(Box::new(ArcData {
            data_ref_count: AtomicUsize::new(1),
            alloc_ref_count: AtomicUsize::new(1),
            data: UnsafeCell::new(Some(data)),
        }));
        Self {
            weak: Weak {
                ptr: NonNull::from(ptr),
            },
        }
    }

    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        if arc.weak.data().alloc_ref_count.load(Relaxed) == 1 {
            fence(Acquire);
            // safety: because alloc_ref_count = 1, this is not referenced by any other Arc
            let arcdata = unsafe { arc.weak.ptr.as_mut() };
            let option = arcdata.data.get_mut();
            let data = option.as_mut().unwrap();
            Some(data)
        } else {
            None
        }
    }

    pub fn downgrade(arc: &Self) -> Weak<T> {
        arc.weak.clone()
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let ptr = self.weak.data().data.get();

        // safety: because arc exists, the data must exist
        unsafe { (*ptr).as_ref().unwrap() }
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        if self.data().alloc_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            abort();
        }
        Self { ptr: self.ptr }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        let weak = self.weak.clone();
        if weak.data().data_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            abort();
        }
        Self { weak }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        if self.data().alloc_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr())); // take ownership of the Box and drop it
            }
        }
    }
}

// note: when arc is dropped, weak is also dropped because it is a member of arc
impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.weak.data().data_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            let ptr = self.weak.data().data.get();

            // safety: because data_ref_count = 0, this is not referenced by any other Arc
            unsafe { *ptr = None }
        }
    }
}

#[test]
fn test() {
    static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);
    struct DetectDrop;
    impl Drop for DetectDrop {
        fn drop(&mut self) {
            NUM_DROPS.fetch_add(1, Relaxed);
        }
    }
    // この時点では weak ポイントはアップグレード可能
    let x = Arc::new(("hello", DetectDrop));
    let y = Arc::downgrade(&x);
    let z = Arc::downgrade(&x);

    let t = std::thread::spawn(move || {
        // この時点では weak ポイントはアップグレード可能
        let y = y.upgrade().unwrap();
        assert_eq!(y.0, "hello");
    });
    assert_eq!(x.0, "hello");
    t.join().unwrap();

    // データはまだドロップされていないはずなので、weakポインタはアップグレード可能
    assert_eq!(NUM_DROPS.load(Relaxed), 0);
    let a = z.upgrade();
    assert!(a.is_some());
    drop(a);

    drop(x);
    // データはドロップされているはずなので、weakポインタはアップブレード不可能
    assert_eq!(NUM_DROPS.load(Relaxed), 1);
    assert!(z.upgrade().is_none());
}
