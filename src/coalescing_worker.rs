use anyhow::{anyhow, Context, Result};
use futures::Future;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::oneshot;
use tokio::{sync::mpsc, task::JoinSet};

// Allows running a number of concurrent operations that take an `A` and produce a `B` making sure
// that for identical `A`s work is only done once.
#[derive(Clone)]
pub struct CoalescingWorker<A, B> {
    sender: mpsc::Sender<(A, oneshot::Sender<Result<B>>)>,
}

impl<A, B> CoalescingWorker<A, B>
where
    A: Clone + std::cmp::Eq + std::hash::Hash + std::marker::Send + std::marker::Sync + 'static,
    B: std::marker::Send + Clone + 'static,
{
    pub fn new<F, Fut>(concurrency: usize, work_fn: F) -> Self
    where
        F: Fn(A) -> Fut + std::marker::Send + 'static,
        Fut: Future<Output = Result<B>> + std::marker::Send + 'static,
    {
        let (sender, mut reciever) = mpsc::channel::<(A, oneshot::Sender<Result<B>>)>(1);

        tokio::spawn(async move {
            let mut set = JoinSet::new();
            let in_flight = Arc::new(Mutex::new(HashMap::new()));

            while let Some((work_item, done_sender)) = reciever.recv().await {
                let in_flight_clone = in_flight.clone();
                {
                    let mut in_flight_guard = in_flight.lock().unwrap();
                    let waiters: &mut Vec<oneshot::Sender<Result<B>>> =
                        in_flight_guard.entry(work_item.clone()).or_default();
                    waiters.push(done_sender);
                    // Only spawn a worker when there is not one for this item.
                    if waiters.len() > 0 {
                        set.spawn({
                            let fut = work_fn(work_item.clone());
                            async move {
                                let output = fut.await;

                                let to_notify = {
                                    let mut in_flight = in_flight_clone.lock().unwrap();
                                    in_flight.remove(&work_item).unwrap_or_default()
                                };
                                match output {
                                    Ok(id) => {
                                        for done_sender in to_notify {
                                            let _ = done_sender.send(Ok(id.clone()));
                                        }
                                    }
                                    Err(e) => {
                                        for done_sender in to_notify {
                                            let _ = done_sender.send(Err(anyhow!("{:?}", e)));
                                        }
                                    }
                                }
                            }
                        });
                    }
                }
                if set.len() >= concurrency {
                    set.join_next().await;
                }
            }
        });
        CoalescingWorker { sender }
    }
    pub async fn do_work(&self, work_item: A) -> Result<B> {
        let (sender, receiver) = oneshot::channel();
        self.sender.send((work_item, sender)).await?;

        receiver.await.with_context(|| format!("recv dropped"))?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;
    use futures::stream;
    use futures::StreamExt;

    #[tokio::test]
    async fn test1() {
        let d = Arc::new(Mutex::new(0));
        let concurrency = 3;

        let c = CoalescingWorker::new(concurrency, move |id: GPhotoItemId| {
            let d = d.clone();
            let id = id.clone();
            async move {
                let rv = {
                    let mut x = d.lock().unwrap();
                    *x += 1;
                    if *x > concurrency {
                        Err(anyhow!("wrong concurrency"))
                    } else {
                        Ok(ImmichItemId(id.0))
                    }
                };
                tokio::time::sleep(std::time::Duration::from_millis(30)).await;
                {
                    let mut x = d.lock().unwrap();
                    *x -= 1;
                }
                rv
            }
        });
        println!("awaiting result");
        let z = stream::iter(vec![1, 2, 3, 4, 5, 6])
            .map(|n| {
                let c = c.clone();
                async move { c.do_work(GPhotoItemId(format!("{}", n))).await }
            })
            .buffer_unordered(100)
            .collect::<Vec<_>>()
            .await;
        println!("result: {:?}", z);
        assert!(z.iter().all(|e| e.is_ok()));
    }

    #[tokio::test]
    async fn test_collate() {
        // Tests that identical ids are only run through once. Here we have only two distinct input ids,
        // 1 and 6, but 4 overall work items. We set concurrency to 3 (smaller than the number of work items),
        // but expect that only max 2 will ever run concurrently due to coalescence.
        let d = Arc::new(Mutex::new(0));
        let concurrency = 3;

        let c = CoalescingWorker::new(concurrency, move |id: GPhotoItemId| {
            let d = d.clone();
            let id = id.clone();
            async move {
                let rv = {
                    let mut x = d.lock().unwrap();
                    *x += 1;
                    if *x > 2 {
                        Err(anyhow!("wrong concurrency"))
                    } else {
                        Ok(ImmichItemId(id.0))
                    }
                };
                tokio::time::sleep(std::time::Duration::from_millis(30)).await;
                {
                    let mut x = d.lock().unwrap();
                    *x -= 1;
                }
                rv
            }
        });
        println!("awaiting result");
        let z = stream::iter(vec![1, 1, 1, 6])
            .map(|n| {
                let c = c.clone();
                async move { c.do_work(GPhotoItemId(format!("{}", n))).await }
            })
            .buffer_unordered(100)
            .collect::<Vec<_>>()
            .await;
        println!("result: {:?}", z);
        assert!(z.iter().all(|e| e.is_ok()));
    }
}
