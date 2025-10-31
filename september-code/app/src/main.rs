use actix_web::{web, App, HttpServer};
use http::{configure, AppState};
use model::MyObject;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        objects: Arc::new(Mutex::new(vec![
            MyObject { id: 1, name: "Initial Object 1".to_string() },
            MyObject { id: 2, name: "Initial Object 2".to_string() },
        ])),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


