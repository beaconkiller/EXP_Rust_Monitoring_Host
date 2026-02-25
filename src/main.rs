mod constant;
mod controllers;
mod global;
mod helper;
mod models;
mod services;
mod worker;

use std::sync::{Arc, Mutex};

use crate::global::LL_global;
use crate::worker::wk_client::StrClientData;

#[tokio::main]
async fn main() {
    LL_global::LLGlobal::set_global();

    let cl_a = Arc::new(worker::wk_client::WkClients::new(
        "192.168.100.205:2109".to_string(),
    ));

    cl_a.clone().init_worker();

    {
        let mut arr_clients = LL_global::GL_ARR_CLIENTS.lock().unwrap();
        arr_clients.push(cl_a);
    }

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
