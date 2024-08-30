// external libraries
use dotenv::dotenv;
use tokio;
use tokio::net::TcpListener;
use axum::routing::{ Router, MethodRouter, get, post, put };
// use axum::middleware::from_fn_with_state;
// use sqlx::{ MySql, Pool, };
// use sqlx::mysql::MySqlPoolOptions;
// standard library
use std::env;
use std::convert::Infallible;
// use std::sync::Arc;
// internal modules
mod pages;

pub struct NavTab {
    pub path: &'static str,
    pub display: &'static str,
}

// pub struct AppState {
//     db: Pool<MySql>,
// }

const ADDR: &'static str = "127.0.0.1:3000";
const NAV_TAB_COUNT: usize = 2;

pub const NAV_TABS: &'static [NavTab; NAV_TAB_COUNT] = &[
    pages::home::NAV_TAB,
    pages::venues::NAV_TAB,
];

#[tokio::main]
async fn main() {

    let nav_method_routers: [MethodRouter<(), Infallible>; NAV_TAB_COUNT] = [
        get(pages::home::home),
        get(pages::venues::venues),
    ];

    dotenv().ok();

    let instagram_app_id: &str = &env::var("INSTAGRAM_APP_ID").expect("instagram app id env");

    // let db_url: &str = &env::var("DB_URL").expect("db url env");

    // let db: Pool<MySql> = MySqlPoolOptions::new()
    //     .connect(db_url)
    //     .await
    //     .expect("database connection");

    // let state: Arc<AppState> = Arc::new(AppState {
    //     db,
    // });

    let mut router: Router = Router::new(); 

    for (i, method_router) in nav_method_routers.into_iter().enumerate() {
        router = router.route(NAV_TABS[i].path, method_router)
    }

    router = router
        .route("/calendar/:year/:month/", get(pages::calendar::calendar))
        .route("/calendar/month/:year/:month/", get(pages::calendar::month));
    //     // .layer(cors)
    //     ;//.with_state(state.clone());

    let listener: TcpListener = TcpListener::bind(ADDR)
        .await
        .expect("tcp listener");

    println!("Server listening on {:#?}", listener.local_addr().expect("local addr"));

    axum::serve(listener, router)
        .await
        .expect("axum serve");

    
}
