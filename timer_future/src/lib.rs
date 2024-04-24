use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// Shared state between the future and the waiting thread
struct SharedState {
    /// Whether or not the sleep time has elapsed
    completed: bool,

    /// The waker for the task that `TimerFuture` is running on.
    /// The thread can use this after setting `completed = true` to tell
    /// `TimerFuture`'s task to wake up, see that `completed = true`, and
    /// move forward.
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // Look at the shared state to see if the timer has already completed.
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            println!("ready \n");
            Poll::Ready(())
        } else {
            println!("set waker");
            // Set the waker so the thread can wake up the current task
            // when the timer has completed.
            shared_state.waker = Some(cx.waker().clone());

            Poll::Pending
            // Poll::Ready(()) this will panic because poll will be called again after Ready
        }
    }
}

impl Drop for TimerFuture {
    fn drop(&mut self) {
        println!("TimerFuture is dropped");
    }
}

impl TimerFuture {
    /// Create a new `TimerFuture` which will complete after the provided
    /// timeout.
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // Spawn the new thread
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            {
                println!("start task 1");
                thread::sleep(duration); // this represents heavy task
                println!("done task 1");
                let mut shared_state = thread_shared_state.lock().unwrap();
                if let Some(waker) = shared_state.waker.take() {
                    println!("\ncall waker to be checked by poll\n");
                    waker.wake();
                }
            }

            {
                println!("start task 2");
                thread::sleep(duration); // this represents heavy task
                println!("done task 2");
                let mut shared_state = thread_shared_state.lock().unwrap();
                shared_state.completed = true; // finaly it's finished
                if let Some(waker) = shared_state.waker.take() {
                    println!("\ncall waker to be checked by poll\n");
                    waker.wake();
                }
            }
        });

        TimerFuture { shared_state }
    }
}
