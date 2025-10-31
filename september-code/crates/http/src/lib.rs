use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use std::sync::{Arc, Mutex};

use model::MyObject;

pub struct AppState {
    pub objects: Arc<Mutex<Vec<MyObject>>>,
}

#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/objects")]
pub async fn get_all_objects(data: web::Data<AppState>) -> impl Responder {
    let objects = data.objects.lock().unwrap();
    HttpResponse::Ok().json(&*objects)
}

#[get("/objects/{id}")]
pub async fn get_object(data: web::Data<AppState>, path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let objects = data.objects.lock().unwrap();
    if let Some(obj) = objects.iter().find(|o| o.id == id) {
        HttpResponse::Ok().json(obj)
    } else {
        HttpResponse::NotFound().body(format!("No object found with id: {}", id))
    }
}

#[post("/objects")]
pub async fn create_object(data: web::Data<AppState>, obj: web::Json<MyObject>) -> impl Responder {
    let mut objects = data.objects.lock().unwrap();
    objects.push(obj.0.clone());
    HttpResponse::Ok().json(obj.0)
}

#[put("/objects/{id}")]
pub async fn update_object(
    data: web::Data<AppState>,
    path: web::Path<u32>,
    obj_update: web::Json<MyObject>,
) -> impl Responder {
    let id = path.into_inner();
    let mut objects = data.objects.lock().unwrap();
    if let Some(pos) = objects.iter().position(|o| o.id == id) {
        objects[pos] = obj_update.0.clone();
        HttpResponse::Ok().json(objects[pos].clone())
    } else {
        HttpResponse::NotFound().body(format!("No object found with id: {}", id))
    }
}

#[delete("/objects/{id}")]
pub async fn delete_object(data: web::Data<AppState>, path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let mut objects = data.objects.lock().unwrap();
    if let Some(pos) = objects.iter().position(|o| o.id == id) {
        let deleted_obj = objects.remove(pos);
        HttpResponse::Ok().json(json!({"deleted": deleted_obj}))
    } else {
        HttpResponse::NotFound().body(format!("No object found with id: {}", id))
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
        .service(echo)
        .service(get_all_objects)
        .service(get_object)
        .service(create_object)
        .service(update_object)
        .service(delete_object)
        .route("/hey", web::get().to(manual_hello));
}


