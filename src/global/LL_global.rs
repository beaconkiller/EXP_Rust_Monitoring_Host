use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex},
};

use crate::services::service_client_control::SrvCLientControl;
use crate::worker::wk_client::WkClients;
use serde_json::Value;

pub static GL_ARR_CLIENTS: LazyLock<Mutex<Vec<Arc<WkClients>>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

pub static GL_ARR_GET_DATA: LazyLock<Mutex<Vec<HashMap<String, Value>>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

pub static GL_SRV_CLIENT_CONTROL: LazyLock<Arc<SrvCLientControl>> = LazyLock::new(|| {
    Arc::new(SrvCLientControl {
        status: Arc::new(tokio::sync::Mutex::new(false)),
        clients: Arc::new(tokio::sync::Mutex::new(vec![])),
        workers: Arc::new(tokio::sync::Mutex::new(vec![])),
    })
});

#[derive(Debug)]
pub struct LLGlobal;

impl LLGlobal {
    pub fn set_global() {
        // GL_ARR_CLIENTS.lock().unwrap();
        // GL_ARR_GET_DATA.lock().unwrap();
    }
}
