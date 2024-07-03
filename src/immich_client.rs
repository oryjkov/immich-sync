use std::{
    ops::Deref,
    sync::{Arc, Condvar, Mutex},
};

use immich_api::apis::configuration::{ApiKey, Configuration};

// ImmichClient takes care of keeping a limited set of ImmichApi clients and
// handing them out using ApiConfigWrapper objects.
#[derive(Clone, Debug)]
pub struct ImmichClient {
    api_configs: Arc<Mutex<Vec<Box<Configuration>>>>,
    configs_empty: Arc<Condvar>,
}

pub struct ApiConfigWrapper<'a> {
    api_config: Option<Box<Configuration>>,
    return_to: &'a ImmichClient,
}

impl<'a> Drop for ApiConfigWrapper<'a> {
    fn drop(&mut self) {
        let ic = self.return_to;

        ic.api_configs
            .lock()
            .unwrap()
            .push(self.api_config.take().unwrap());
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
    pub fn new(n: usize, immich_url: &str, api_key: Option<ApiKey>) -> Self {
        ImmichClient {
            api_configs: Arc::new(Mutex::new(
                [0..n]
                    .iter()
                    .map(|_| {
                        Box::new(Configuration {
                            api_key: api_key.clone(),
                            base_path: immich_url.to_string(),
                            ..Default::default()
                        })
                    })
                    .collect(),
            )),
            configs_empty: Arc::new(Condvar::new()),
        }
    }
    pub fn get_config(&self) -> ApiConfigWrapper {
        let api_config = {
            let mut g = self.api_configs.lock().unwrap();
            loop {
                match g.pop() {
                    Some(api_config) => {
                        break api_config;
                    }
                    None => {
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
}
