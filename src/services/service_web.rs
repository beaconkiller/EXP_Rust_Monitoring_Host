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
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};

pub struct SrvWeb;

impl SrvWeb {
    pub async fn init() {
        let config: StrConfig = VarConstant::get_config();

        // =====================================================================
        // ======================== ASSIGNING THE ROUTES ========================
        // =====================================================================

        let mut api_routes: Router = Router::new();
        api_routes = SrvRoutes::apply_routes(api_routes);

        // =====================================================================
        // ======================= ASSIGN THE API PREFIX =======================
        // =====================================================================
        // ---------------------------------------------------------------------
        // -- And also the web directory that contains a built web app, which --
        // -- has index.html file inside. I'm still learning about these      --
        // -- chained functions. The functions after the .nest() is totally   --
        // -- optional, use that if One wants to deploy a self contained app. --
        // ---------------------------------------------------------------------

        let mut app: Router = Router::new().nest("/api", api_routes).fallback_service(
            ServeDir::new("src/web").not_found_service(ServeFile::new("src/web/index.html")),
        );

        // =====================================================================
        // ======================= ASSIGN THE MIDDLEWARE =======================
        // =====================================================================

        app = app.layer(middleware::from_fn(SrvRoutes::api_handler));

        // =====================================================================
        // ====================== ASSIGN THE CORS CONFIG =======================
        // =====================================================================

        let cors = CorsLayer::permissive();
        // let cors = CorsLayer::new().allow_origin(Any);
        app = app.layer(cors);

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

        println!("{:?}", app);
        println!("{:?}", listener);

        axum::serve(listener, app).await.expect("Failed")
    }
}
