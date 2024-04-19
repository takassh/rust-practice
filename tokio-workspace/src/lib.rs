#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use tokio::{
        sync::{broadcast, mpsc, watch, Mutex},
        time::sleep,
    };

    use super::*;

    #[tokio::test]
    async fn mpsc() {
        let (tx, rx) = mpsc::channel(100);

        tokio::spawn(async move {
            let mut i = 0;
            loop {
                sleep(Duration::from_secs(1)).await;
                if let Err(_) = tx.send(i).await {
                    println!("receiver dropped");
                    return;
                }

                i += 1;
            }
        });

        let rx = Arc::new(Mutex::new(rx));
        let _rx = rx.clone();
        let worker1 = tokio::spawn(async move {
            loop {
                let result = _rx.lock().await.recv().await;
                println!("worker1 got = {:?}", result);
                sleep(Duration::from_secs(1)).await;
            }
        });

        let _rx = rx.clone();
        let worker2 = tokio::spawn(async move {
            loop {
                let result = _rx.lock().await.recv().await;
                println!("worker2 got = {:?}", result);
                sleep(Duration::from_secs(5)).await;
            }
        });

        // worker1 consumes values 5x faster than worker2
        // each value is consumed once
        let _ = worker1.await;
        let _ = worker2.await;
    }

    #[tokio::test]
    async fn broadcast() {
        let (tx, mut rx) = broadcast::channel(100);
        let mut rx2 = tx.subscribe();

        tokio::spawn(async move {
            let mut i = 0;
            loop {
                sleep(Duration::from_secs(1)).await;
                if let Err(_) = tx.send(i) {
                    println!("receiver dropped");
                    return;
                }

                i += 1;
            }
        });

        let worker1 = tokio::spawn(async move {
            loop {
                let result = rx.recv().await;
                println!("worker1 got = {:?}", result);
                sleep(Duration::from_secs(1)).await;
            }
        });

        let worker2 = tokio::spawn(async move {
            loop {
                let result = rx2.recv().await;
                println!("worker2 got = {:?}", result);
                sleep(Duration::from_secs(5)).await;
            }
        });

        // worker1 consumes values 5x faster than worker2
        // each value is consumed by each worker
        let _ = worker1.await;
        let _ = worker2.await;
    }

    #[tokio::test]
    async fn watch() {
        let (tx, mut rx) = watch::channel(0);
        let mut rx2 = tx.subscribe();

        tokio::spawn(async move {
            let mut i = 0;
            loop {
                sleep(Duration::from_secs(1)).await;
                if let Err(_) = tx.send(i) {
                    println!("receiver dropped");
                    return;
                }

                i += 1;
            }
        });

        let worker1 = tokio::spawn(async move {
            loop {
                println!("worker1 got = {:?}", *rx.borrow_and_update());
                let _ = rx.changed().await;
                sleep(Duration::from_secs(1)).await;
            }
        });

        let worker2 = tokio::spawn(async move {
            loop {
                println!("worker2 got = {:?}", *rx2.borrow_and_update());
                let _ = rx2.changed().await;
                sleep(Duration::from_secs(5)).await;
            }
        });

        // worker1 consumes values 5x faster than worker2
        // each value is consumed by each worker but it can be skipped
        let _ = worker1.await;
        let _ = worker2.await;
    }
}

// mpsc
// M (a,b,c,d) -> S
// each value is sent without clone
// you can send value multiple times
// But S can be clone by Arc, so S can consume them by separate threads.
// If you want to consume each value once, you can use mpsc

// broadcast
// M (a,b,c,d) -> S (e,f,g,h)
// each value is cloned and sent
// you can send value multiple times
// If you want to consume each value on each cumsumers, you can use broadcast

// watch
// M -> S (e,f,g,h)
// each value is cloned and sent but it can be skipped
// you can send value multiple times

// oneshot
// M -> S
// each value is sent
// you can send value once
// one value can be sent
