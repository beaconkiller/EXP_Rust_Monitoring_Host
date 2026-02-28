use std::{collections::HashMap, sync::Arc};

use axum::{Json, extract::Query};
use reqwest::get;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    controllers::cont_get_info::StrGetInfo, global::LL_global::GL_SRV_CLIENT_CONTROL,
    models::model_api_response::ApiResponse, services::service_client_control::SrvCLientControl,
    worker::wk_client::WkClients,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrWorkerStatus {
    pub addr: Option<String>,
    pub status: Option<i32>,
    pub last_success: Option<String>,
}

pub struct ContWorker;

impl ContWorker {
    pub async fn get_workers(
        Query(params): Query<HashMap<String, Value>>,
    ) -> Json<ApiResponse<Vec<StrWorkerStatus>>> {
        println!("{:?}", params);

        let SrvClCtl = &GL_SRV_CLIENT_CONTROL;
        let arr_workers: Vec<StrWorkerStatus> = SrvClCtl.get_all_workers().await;

        let resp: ApiResponse<Vec<StrWorkerStatus>> = ApiResponse {
            data: arr_workers,
            status: 200,
            message: "Success".to_string(),
        };

        Json(resp)
    }

    pub async fn add_new_addr(
        Query(params): Query<HashMap<String, Value>>,
        body: String,
    ) -> Json<ApiResponse<Vec<String>>> {
        println!("{:?}", " ================================");
        println!("{:?}", " ========= add_new_addr =========");
        println!("{:?}", " ================================");
        println!("{:?}", params);
        println!("{:?}", body);

        let body_parsed: Value = serde_json::from_str(&body).unwrap();
        println!("{:?}", body_parsed);

        let addr = body_parsed["addr"].as_str().unwrap_or_default().to_string();
        println!("{:?}", " ---- new addr ---- ");
        println!("{:?}", addr);

        let SrvClCtl = &GL_SRV_CLIENT_CONTROL;
        SrvClCtl.add_worker(vec![addr]).await;

        let arr_addr: Vec<String> = vec!["test".to_string()];
        let resp = ApiResponse {
            data: arr_addr,
            status: 200,
            message: "Success".to_string(),
        };

        Json(resp)
    }
}
