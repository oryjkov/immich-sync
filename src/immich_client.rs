use log::debug;
use std::{
    ops::Deref,
    sync::{Arc, Condvar, Mutex},
};

use immich_api::apis::configuration::{ApiKey, Configuration};

use crate::types::{ImmichAlbumId, ImmichItemId};

// ImmichClient takes care of keeping a limited set of ImmichApi clients and
// handing them out using ApiConfigWrapper objects.
#[derive(Clone, Debug)]
pub struct ImmichClient {
    api_configs: Arc<Mutex<Vec<Box<Configuration>>>>,
    configs_empty: Arc<Condvar>,
    pub read_only: bool,
    base_url: String,
}

pub struct ApiConfigWrapper<'a> {
    api_config: Option<Box<Configuration>>,
    return_to: &'a ImmichClient,
}

impl<'a> Drop for ApiConfigWrapper<'a> {
    fn drop(&mut self) {
        let ic = self.return_to;

        {
            let mut g = ic.api_configs.lock().unwrap();
            g.push(self.api_config.take().unwrap());
            debug!("returned immich api config, remaining: {}", g.len());
        }
        ic.configs_empty.notify_one();
    }
}

impl<'a> Deref for ApiConfigWrapper<'a> {
    type Target = Configuration;
    fn deref(&self) -> &Self::Target {
        self.api_config.as_ref().unwrap()
    }
}

impl ImmichClient {
    pub fn new(n: usize, immich_url: &str, api_key: Option<ApiKey>, read_only: bool) -> Self {
        ImmichClient {
            api_configs: Arc::new(Mutex::new(vec![
                {
                    Box::new(Configuration {
                        api_key: api_key,
                        base_path: immich_url.to_string(),
                        ..Default::default()
                    })
                };
                n
            ])),
            configs_empty: Arc::new(Condvar::new()),
            read_only,
            base_url: immich_url.strip_suffix("/api").unwrap().to_string(),
        }
    }
    pub fn item_url(&self, item_id: &ImmichItemId) -> String {
        format!("{}/photos/{}", self.base_url, item_id.0)
    }
    pub fn album_url(&self, album_id: &ImmichAlbumId) -> String {
        format!("{}/albums/{}", self.base_url, album_id.0)
    }
    pub fn get_config(&self) -> ApiConfigWrapper {
        let api_config = {
            let mut g = self.api_configs.lock().unwrap();
            loop {
                match g.pop() {
                    Some(api_config) => {
                        debug!("took immich api config, remaining: {}", g.len());
                        break api_config;
                    }
                    None => {
                        debug!("ran out of immich api configs, {}", g.len());
                        g = self.configs_empty.wait(g).unwrap();
                    }
                }
            }
        };

        ApiConfigWrapper {
            api_config: Some(api_config),
            return_to: &self,
        }
    }
    pub fn get_config_for_writing(&self) -> anyhow::Result<ApiConfigWrapper> {
        if self.read_only {
            return Err(anyhow::anyhow!("asked for writing with a read-only config"));
        }
        let api_config = {
            let mut g = self.api_configs.lock().unwrap();
            loop {
                match g.pop() {
                    Some(api_config) => {
                        debug!("took immich api config, remaining: {}", g.len());
                        break api_config;
                    }
                    None => {
                        debug!("ran out of immich api configs, {}", g.len());
                        g = self.configs_empty.wait(g).unwrap();
                    }
                }
            }
        };

        Ok(ApiConfigWrapper {
            api_config: Some(api_config),
            return_to: &self,
        })
    }
}
