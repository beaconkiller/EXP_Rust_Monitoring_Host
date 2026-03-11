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

    // =================================================
    // ============= STARTING HOST SERVICE =============
    // =================================================

    // ----- OPTIONALLY ADD INITIALIZATION WORKER ------

    GL_SRV_CLIENT_CONTROL
        .add_client(vec![
            // "127.0.0.1:2109".to_string(),
            "202.74.75.24:2109".to_string(),
        ])
        .await;

    // --------- ACTUALLY STARTING THE WORKER ----------

    GL_SRV_CLIENT_CONTROL.init().await;

    // =================================================
    // ============= STARTING WEB SERVICE ==============
    // =================================================

    services::service_web::SrvWeb::init().await;
}
