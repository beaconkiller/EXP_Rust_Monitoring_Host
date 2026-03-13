use std::collections::HashMap;

use axum::{Json, extract::Query};
use serde_json::Value;

use crate::models::model_api_response::ApiResponse;

pub struct ContSessTransit;

impl ContSessTransit {

    pub async fn get_qr_session(
        body:String, Query(params): Query<HashMap<String, Value>>,
    ) -> Json<ApiResponse<String>> {
 
        Json(ApiResponse { 
            status: 200, 
            data: "asd".to_string(), 
            message: "test".to_string() 
        })
    }

}
