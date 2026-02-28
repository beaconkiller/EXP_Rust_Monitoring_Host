use crate::{
    constant::var_constant::{StrConfig, VarConstant},
    controllers::{cont_get_info::ContGetInfo, cont_worker::ContWorker},
    services::service_routes::SrvRoutes,
};

use axum::{
    Router, middleware,
    routing::{get, post},
};
use tokio;

pub struct SrvWeb;

impl SrvWeb {
    pub async fn init() {
        let config: StrConfig = VarConstant::get_config();

        let mut app: Router = Router::new();

        // =====================================================================
        // ======================== ASSIGNING THE ROUTES ========================
        // =====================================================================

        // let arr_routes: Vec<fn(Router) -> Router> = service_routes::SrvRoutes::get_routes();

        app = app.route("/get_info_by_addr", get(ContGetInfo::get_info_by_addr));
        app = app.route("/get_workers", get(ContWorker::get_workers));
        app = app.route("/add_new_addr", post(ContWorker::add_new_addr));

        // =====================================================================
        // ======================= ASSIGN THE MIDDLEWARE =======================
        // =====================================================================

        app = app.layer(middleware::from_fn(SrvRoutes::api_handler));

        // =====================================================================
        // ====================== STARTING THE WEB SERVER ======================
        // =====================================================================

        let addr: String = config.addr;
        let port: String = config.port.to_string();

        let full_addr: String = format!("{addr}:{port}");

        let listener = match tokio::net::TcpListener::bind(full_addr).await {
            Ok(l) => l,
            Err(e) => {
                println!("Failed : {}", e);
                return;
            }
        };

        println!("{:?}", listener);

        axum::serve(listener, app).await.expect("Failed")
    }
}
