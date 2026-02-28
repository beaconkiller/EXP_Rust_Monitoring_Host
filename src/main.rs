mod constant;
mod controllers;
mod global;
mod helper;
mod models;
mod services;
mod worker;

use chrono::{self, DateTime, Utc};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use crate::global::LL_global::{self, GL_SRV_CLIENT_CONTROL};
use crate::services::service_client_control::SrvCLientControl;
use crate::worker::wk_client::StrClientData;

#[tokio::main]
async fn main() {
    let time = SystemTime::now();
    let dt: DateTime<Utc> = time.clone().into();
    let ISOstr = format!("{}", dt.format("%+"));

    println!("{:?}", time);
    println!("{:?}", ISOstr);

    LL_global::LLGlobal::set_global();

    // let arr_clients: Vec<String> = vec![
    //     "192.168.100.205:2109".to_string(),
    //     "192.168.100.68:2109".to_string(),
    // ];

    // let SrvClCtl = GL_SRV_CLIENT_CONTROL.clone();
    // SrvClCtl.add_client(arr_clients).await;
    // SrvClCtl.init().await;

    // SrvClCtl.get_all_data().await;

    services::service_web::SrvWeb::init().await;
}
