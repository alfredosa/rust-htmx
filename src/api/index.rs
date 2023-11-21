use std::collections::HashMap;

use actix_web::HttpResponse;

pub async fn index() -> HttpResponse {
    let mut data = HashMap::new();
    data.insert("title", "Hello World!");
    data.insert("name", "Actix-web");

    HttpResponse::Ok()
        .content_type("text/html")
        .body("Hello World!")
}
