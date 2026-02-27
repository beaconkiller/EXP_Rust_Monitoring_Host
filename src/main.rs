mod constant;
mod controllers;
mod global;
mod helper;
mod models;
mod services;
mod worker;

use std::sync::{Arc, Mutex};

use crate::global::LL_global::{self, GL_SRV_CLIENT_CONTROL};
use crate::services::service_client_control::SrvCLientControl;
use crate::worker::wk_client::StrClientData;

#[tokio::main]
async fn main() {
    LL_global::LLGlobal::set_global();

    let arr_clients: Vec<String> = vec![
        "192.168.100.205:2109".to_string(),
        "192.168.100.68:2109".to_string(),
    ];

    let SrvClCtl = GL_SRV_CLIENT_CONTROL.clone();
    SrvClCtl.add_client(arr_clients).await;
    SrvClCtl.init().await;

    services::service_web::SrvWeb::init().await;

    // loop {
    //     cl_a.get_latest_data();
    //     tokio::time::sleep(Duration::from_secs(1)).await;
    // }

    // SrvCLientControl::add_client("192.168.100.205:2109");
    // SrvCLientControl::add_client("192.168.100.205:3200");

    // println!("{:?}", LL_global::GL_ARR_CLIENTS.lock().unwrap());

    // let x = SrvCLientControl::init().await;
}
