#![allow(unused)]

use axum::{Router, response::{Html, IntoResponse, Response}, routing::{get, get_service}, extract::{Query, Path}, middleware};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use model::ModelController;

pub use self::error::{Error, Result};
mod error;
mod web;
mod model;

#[tokio::main]
async fn main() -> Result<()> {

    let mc = ModelController::new().await?;
    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis) // nest 指分流， routes_apis中 是以 /api 开头的路由
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static()); // 后备匹配，如果 merge 的匹配不上再来匹配后备
        
    println!("axum web running in localhost:9999");

    axum::Server::bind(&"0.0.0.0:9999".parse().unwrap())
        .serve(routes_all.into_make_service())
        .await
        .unwrap();


    Ok(())
}


async fn main_response_mapper(res : Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();

    res
}


fn routes_hello() -> Router {
    Router::new()
    .route("/hello", get(handler_hello))
    .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HellrParmas {
    name : Option<String>,
}

// e.g, `/hello?name=Jen 
async fn handler_hello(Query(params) : Query<HellrParmas>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g., `/hello2/Mike`

async fn handler_hello2(Path(name) : Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 {name:?}", "HANDLER");

    Html(format!("Hello2 <strong> {name} </strong>"))
}



// -------------  Route Static ---------------------

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

