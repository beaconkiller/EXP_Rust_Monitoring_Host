use crate::{
    controllers::cont_get_info::StrGetInfo,
    global::LL_global::{self, GL_SRV_CLIENT_CONTROL},
    models::model_api_response::ApiResponse,
    worker::{self, wk_client::WkClients},
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct SrvCLientControl {
    pub status: Arc<Mutex<bool>>,
    pub clients: Arc<Mutex<Vec<String>>>,
    pub workers: Arc<Mutex<Vec<Arc<WkClients>>>>,
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
        self.add_worker(arr_ip).await;
    }

    pub async fn add_worker(&self, arr_ip: Vec<String>) {
        for el in arr_ip {
            let cl_worker: Arc<WkClients> = Arc::new(worker::wk_client::WkClients::new(el));
            cl_worker.clone().init_worker();
            {
                let mut arr_workers = LL_global::GL_SRV_CLIENT_CONTROL.workers.lock().await;
                arr_workers.push(cl_worker);
            }
        }
    }

    pub async fn add_client(&self, arr_ip: Vec<String>) {
        let mut arr = self.clients.lock().await;
        *arr = arr_ip;
    }

    pub async fn get_all_data(&self) -> Vec<StrGetInfo> {
        let SrvClCtl = GL_SRV_CLIENT_CONTROL.clone();
        let arr_workers = SrvClCtl.workers.lock().await;

        println!("{:?}", " =========== arr_workers =========== ");
        println!("{:?}", arr_workers);

        let mut arr_data: Vec<StrGetInfo> = vec![];
        for el in arr_workers.iter() {
            arr_data.push(el.get_info());
        }

        arr_data
    }
}
