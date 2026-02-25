use axum::{Router, http::Request, middleware::Next, response::Response, routing::get};

use crate::controllers::cont_get_info::ContGetInfo;
pub struct SrvRoutes;

impl SrvRoutes {
    pub fn _get_routes() -> Vec<fn(Router) -> Router> {
        let arr_routes: Vec<fn(Router) -> Router> =
            vec![|v| v.route("/get_info_by_addr", get(ContGetInfo::get_info_by_addr))];

        arr_routes
    }

    pub async fn api_handler(req: Request<axum::body::Body>, next: Next) -> Response {
        println!("{} {}", req.method(), req.uri());

        let response = next.run(req).await;

        response
    }
}
