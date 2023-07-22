use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Package {
    path: String,
}

#[derive(Serialize)]
struct NotFoundResponse {
    message: String,
}

#[derive(Serialize)]
struct ValidationErrorResponse {
    message: String,
}

fn check_valid_path(path: &str) -> bool {
    if path == "test/fail" {
        return false;
    }

    true
}

#[get("/packages")]
async fn get_package_data(package: web::Query<Package>) -> impl Responder {
    let is_valid_path = check_valid_path(&package.path);

    if !is_valid_path {
        let response = ValidationErrorResponse {
            message: "Invalid path".to_string(),
        };
        return HttpResponse::BadRequest().json(response);
    }

    HttpResponse::Ok().body(format!("Hello {}!", package.path))
}

async fn default_not_found_handler() -> HttpResponse {
    let response = NotFoundResponse {
        message: "Not found".to_string(),
    };
    HttpResponse::NotFound().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(get_package_data)
            .default_service(web::route().to(default_not_found_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
