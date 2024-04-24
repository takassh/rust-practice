use futures::{
    future::{join, BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};
use std::{
    future::Future,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::Context,
    time::{Duration, SystemTime},
};
use timer_future::TimerFuture;

/// Task executor that receives tasks off of a channel and runs them.
struct Executor {
    /// this is just a channel to receive the task
    task_queue: Receiver<Arc<Task>>,
}

/// `Spawner` spawns new futures onto the task channel.
#[derive(Clone)]
struct Spawner {
    /// this is just a channel to send the task back to the executor
    task_sender: SyncSender<Arc<Task>>,
}

/// A future that can reschedule itself to be polled by an `Executor`.
struct Task {
    /// The future that should be run to completion.
    /// Actually, the `Mutex` is not necessary because we are not going to access the future from multiple threads.
    /// But, ArcWake requires the future to be `Send` and `Sync`.
    /// Usually, future is executed in many threads.
    future: Mutex<BoxFuture<'static, ()>>,

    /// this is used in wake_by_ref
    /// this is just a channel to send the task back to the executor
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // Maximum number of tasks to allow queueing in the channel at once.
    // This is just to make `sync_channel` happy, and wouldn't be present in
    // a real executor.
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, task_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { task_queue }, Spawner { task_sender })
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(future),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

// This is called when we call `wake()`
// We want to put the task back in the task queue so that it can be polled again.
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

// Drop is called when all tasks are dropped because Task is an Arc.
impl Drop for Task {
    fn drop(&mut self) {
        println!("all tasks are dropped");
    }
}

impl Executor {
    fn run(&self) {
        // Run the executor until all tasks are dropped.
        loop {
            let Ok(task) = self.task_queue.recv() else {
                println!("no task to run");
                return;
            };
            // Create a waker to be called when the task should be awoken
            let waker = waker_ref(&task);
            let context = &mut Context::from_waker(&waker);

            // Take the future from task and poll
            println!("poll the task");
            let _ = task.future.lock().unwrap().as_mut().poll(context);
        }
    }
}

fn main() {
    println!("start of main\n");
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(TimerFuture::new(Duration::new(2, 0)));
    // it takes almost 2 seconds to complete
    // spawner.spawn(async {
    //     let start = SystemTime::now();
    //     join(
    //         TimerFuture::new(Duration::new(2, 0)),
    //         TimerFuture::new(Duration::new(2, 0)),
    //     )
    //     .await;
    //     println!("elapsed:{:?}", start.elapsed().unwrap());
    // });

    // drop the spawner since task is already spwaned and sent to executor
    // if we don't drop spawner, it will be in memory and this proccess will not end
    drop(spawner);

    // Run the executor until the task queue is empty.
    executor.run();

    println!("\nend of main");
}
