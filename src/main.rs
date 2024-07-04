use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

//#[]은 속성 정의
#[derive(Serialize, Deserialize)]
struct Item {
    id: u32,
    name: String,
}

async fn get_items() -> impl Responder {
    let items = vec![
        Item { id: 1, name: "Item 1".to_string()},
        Item { id: 2, name: "Item 2".to_string()},
    ];
    HttpResponse::Ok().json(items)
}

async fn get_item(item_id: web::Path<u32>) -> impl Responder {
    let item = Item { id: *item_id, name: format!("Item {}", item_id)};
    HttpResponse::Ok().json(item)
}

async fn create_item(item: web::Json<Item>) -> impl Responder {
    HttpResponse::Created().json(item.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/items", web::get().to(get_items))
            .route("/items/{id}", web::get().to(get_item))
            .route("/items", web::post().to(create_item))
      
    })
    .bind("localhost:8090") ?
    .run()
    .await
}


//Cargo.toml
[package]
name = "restapi-server"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.0"
serde = { version = "1.0", features = ["derive"]}
serde_json = "1.0"
