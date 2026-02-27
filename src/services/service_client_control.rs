use crate::{
    LL_global::{GL_ARR_CLIENTS, GL_ARR_GET_DATA},
    global::LL_global::{self, GL_SRV_CLIENT_CONTROL, LLGlobal},
    helper::service_helper::SrvHelper,
    worker::{self, wk_client::WkClients},
};
use reqwest::Response;
use serde_json::Value;
use std::{
    clone,
    collections::HashMap,
    sync::{Arc, LazyLock},
    time::Duration,
};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct SrvCLientControl {
    pub status: Arc<Mutex<bool>>,
    pub clients: Arc<Mutex<Vec<String>>>,
    pub workers: Arc<Mutex<Vec<WkClients>>>,
}

impl SrvCLientControl {
    pub fn new() -> Arc<Self> {
        let cl_ctl: Arc<SrvCLientControl> = Arc::new(SrvCLientControl {
            status: Arc::new(Mutex::new(false)),
            clients: Arc::new(Mutex::new(vec![])),
            workers: Arc::new(Mutex::new(vec![])),
        });

        cl_ctl
    }

    pub async fn init(&self) {
        println!();
        println!(" --------- INIT SERVICE --------- ");
        println!();

        let mut clients = self.clients.lock().await;
        let arr_ip = clients.drain(..).collect::<Vec<_>>();
        self.add_worker(arr_ip);
    }

    pub fn add_worker(&self, arr_ip: Vec<String>) {
        for el in arr_ip {
            let cl_a: Arc<WkClients> = Arc::new(worker::wk_client::WkClients::new(el));
            cl_a.clone().init_worker();
            {
                let mut arr_clients = LL_global::GL_ARR_CLIENTS.lock().unwrap();
                arr_clients.push(cl_a);
            }
        }
    }

    pub async fn add_client(&self, arr_ip: Vec<String>) {
        let mut arr = self.clients.lock().await;
        *arr = arr_ip;
    }
}
