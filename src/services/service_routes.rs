use axum::{
    Router,
    http::Request,
    middleware::Next,
    response::Response,
    routing::{get, post},
};

use crate::controllers::{
    cont_get_info::ContGetInfo, cont_session_transit::ContSessTransit, cont_worker::ContWorker,
};
pub struct SrvRoutes;

impl SrvRoutes {
    pub async fn api_handler(req: Request<axum::body::Body>, next: Next) -> Response {
        println!("{} {}", req.method(), req.uri());
        let response = next.run(req).await;
        response
    }

    pub fn apply_routes(mut router: Router) -> Router {
        // ===============================================
        // ==================== ROUTES ===================
        // ===============================================

        // ===============================================
        // =================== SYSINFO ===================
        // ===============================================

        router = router.route("/get_info_by_addr", get(ContGetInfo::get_info_by_addr));
        router = router.route("/get_workers", get(ContWorker::get_workers));
        router = router.route("/add_new_addr", post(ContWorker::add_new_addr));

        // ===============================================
        // =================== QR API ====================
        // ===============================================

        router = router.route("/gen_transit_sess", post(ContSessTransit::get_qr_session));
        // router = router.route("/get_workers", get(ContWorker::get_workers));
        // router = router.route("/add_new_addr", post(ContWorker::add_new_addr));

        // -----------------------------------------------
        // ----------- RETURN THE FINAL ROUTER -----------
        // -----------------------------------------------

        router
    }
}
