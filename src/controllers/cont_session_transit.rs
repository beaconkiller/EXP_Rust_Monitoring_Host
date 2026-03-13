use std::collections::HashMap;

use axum::{Json, extract::Query, http::HeaderMap, response::IntoResponse};
use base64::{Engine as _, engine::general_purpose};
use image::{ColorType, DynamicImage, ImageEncoder};
use reqwest::header;
use serde_json::Value;

use crate::{
    global::LL_global::GL_SRV_TRANSIT_SESS,
    models::model_api_response::ApiResponse,
    services::{service_generate::SrvGenerate, service_transit_sess::SrvTransitSess},
};

pub struct ContSessTransit;

impl ContSessTransit {
    pub async fn get_qr_session(
        Query(params): Query<HashMap<String, Value>>,
        body: String,
    ) -> Json<ApiResponse<String>> {
        println!("{:?}", body);

        let qr_base64 = GL_SRV_TRANSIT_SESS.add_new_session_auto("data").await;

        Json(ApiResponse {
            status: 200,
            data: qr_base64,
            message: "test".to_string(),
        })
    }
}
