use std::sync::{Arc, LazyLock, Mutex};
use tokio::sync;

use crate::services::{
    service_client_control::SrvCLientControl, service_transit_sess::SrvTransitSess,
};

pub static GL_SRV_CLIENT_CONTROL: LazyLock<Arc<SrvCLientControl>> = LazyLock::new(|| {
    Arc::new(SrvCLientControl {
        status: Arc::new(tokio::sync::Mutex::new(false)),
        clients: Arc::new(tokio::sync::Mutex::new(vec![])),
        workers: Arc::new(tokio::sync::Mutex::new(vec![])),
    })
});

pub static GL_SRV_TRANSIT_SESS: LazyLock<Arc<SrvTransitSess>> = LazyLock::new(|| {
    Arc::new(SrvTransitSess {
        transit_data: tokio::sync::Mutex::new(vec![]),
        sess_lifespan: 10,
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
