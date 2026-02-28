use std::collections::HashMap;

use crate::{
    global::LL_global::GL_SRV_CLIENT_CONTROL, models::model_api_response::ApiResponse,
    worker::wk_client::StrClientData,
};
use axum::{Json, extract::Query};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Clone, Debug)]
pub struct StrGetInfo {
    pub addr: String,
    pub status: i32,
    pub data: StrClientData,
}

pub struct ContGetInfo;

impl ContGetInfo {
    pub async fn get_info_by_addr(
        Query(params): Query<HashMap<String, Value>>,
    ) -> Json<ApiResponse<Vec<StrGetInfo>>> {
        println!("{:?}", params);
        let arr_data = GL_SRV_CLIENT_CONTROL.get_all_data().await;

        let resp = ApiResponse {
            status: 200,
            data: arr_data,
            message: "success".to_string(),
        };

        // println!("{:?}", resp);

        Json(resp)
    }
}
