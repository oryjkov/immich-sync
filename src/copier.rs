use crate::gpclient::GPClient;
use crate::types::*;
use anyhow::{anyhow, Context, Result};
use futures::Future;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::oneshot;
use tokio::{sync::mpsc, task::JoinSet};

pub struct Copier {
    in_flight: Arc<Mutex<HashMap<GPhotoItemId, Vec<oneshot::Sender<Result<ImmichItemId>>>>>>,
    sender: mpsc::Sender<(GPhotoItemId, oneshot::Sender<Result<ImmichItemId>>)>,
    server: tokio::task::JoinHandle<()>,
}

impl Copier {
    pub fn new<F, Fut>(copy_one: F) -> Self
    where
        F: Fn(&GPhotoItemId) -> Fut + std::marker::Send + 'static,
        Fut: Future<Output = Result<ImmichItemId>> + std::marker::Send + 'static,
    {
        let (sender, mut reciever) =
            mpsc::channel::<(GPhotoItemId, oneshot::Sender<Result<ImmichItemId>>)>(1);

        let in_flight = Arc::new(Mutex::new(HashMap::new()));
        let in_flight_clone = in_flight.clone();
        let server = tokio::spawn(async move {
            let mut set = JoinSet::new();

            while let Some((gphoto_item_id, done_sender)) = reciever.recv().await {
                let in_flight_clone = in_flight.clone();
                {
                    let mut in_flight_guard = in_flight.lock().unwrap();
                    let waiters: &mut Vec<oneshot::Sender<Result<ImmichItemId>>> =
                        in_flight_guard.entry(gphoto_item_id.clone()).or_default();
                    waiters.push(done_sender);
                    if waiters.len() == 1 {
                        set.spawn({
                            let immich_id_fut = copy_one(&gphoto_item_id);
                            async move {
                                println!("gonna copy {:?}", gphoto_item_id);
                                let immich_id = immich_id_fut.await;

                                let to_notify = {
                                    let mut in_flight = in_flight_clone.lock().unwrap();
                                    in_flight.remove(&gphoto_item_id).unwrap_or_default()
                                };
                                match immich_id {
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
                if set.len() >= 10 {
                    set.join_next().await;
                }
                println!("queued up");
            }
        });
        Copier {
            in_flight: in_flight_clone,
            sender,
            server,
        }
    }
    pub async fn copy_item_to_immich(&self, gphoto_item_id: &GPhotoItemId) -> Result<ImmichItemId> {
        let (sender, receiver) = oneshot::channel();
        self.sender.send((gphoto_item_id.clone(), sender)).await?;

        receiver.await.with_context(|| format!("recv dropped"))?
    }
}

async fn try_one(x: &str, gpclient: GPClient) {
    let c = Copier::new(move |id| {
        let gpclient = gpclient.clone();
        async move {
            gpclient
                .get_album(&GPhotoAlbumId("tmp".to_string()))
                .await?;
            Ok(ImmichItemId("123".to_string()))
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test1() {
        let c = Copier::new(move |id| async move {
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            Ok(ImmichItemId("123".to_string()))
        });
        println!("awaiting result");
        let z = c
            .copy_item_to_immich(&GPhotoItemId("321".to_string()))
            .await;
        println!("result: {:?}", z);
    }
}
