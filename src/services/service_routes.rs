use axum::{
    Router,
    http::Request,
    middleware::Next,
    response::Response,
    routing::{get, post},
};

use crate::controllers::{cont_get_info::ContGetInfo, cont_worker::ContWorker};
pub struct SrvRoutes;

impl SrvRoutes {
    pub fn _get_routes() -> Vec<fn(Router) -> Router> {
        let arr_routes: Vec<fn(Router) -> Router> = vec![
            |v| v.route("/get_info_by_addr", get(ContGetInfo::get_info_by_addr)),
            |v| v.route("/get_workers", get(ContWorker::get_workers)),
            |v| v.route("/add_new_addr", post(ContWorker::add_new_addr)),
        ];

        arr_routes
    }

    pub async fn api_handler(req: Request<axum::body::Body>, next: Next) -> Response {
        println!("{} {}", req.method(), req.uri());

        let response = next.run(req).await;

        response
    }
}
