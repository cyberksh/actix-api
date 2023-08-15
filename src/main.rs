use std::collections::HashMap;
use std::fs;

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Package {
    path: String,
}

#[derive(Serialize, Debug)]
struct Metadata {
    name: String,
    _type: String,
}

#[derive(Serialize, Debug)]
struct Data {
    metadata: String,
    data: HashMap<String, Metadata>,
}

#[get("/packages")]
async fn get_package_data(package: web::Query<Package>) -> impl Responder {
    let paths = fs::read_dir(&package.path).unwrap();

    let mut children = HashMap::new();

    for path in paths {
        let safe_path = path.unwrap();
        let path_type = {
            if safe_path.file_type().unwrap().is_dir() {
                String::from("dir")
            } else if safe_path.file_type().unwrap().is_file() {
                String::from("file")
            } else if safe_path.file_type().unwrap().is_symlink() {
                String::from("link")
            } else {
                String::from("unknown")
            }
        };

        children.insert(
            safe_path.path().to_str().unwrap().to_string(),
            Metadata {
                name: safe_path.file_name().to_str().unwrap().to_string(),
                _type: path_type,
            },
        );
    }

    let response = Data {
        // How do I do this without clone?
        metadata: package.path.clone(),
        data: children,
    };

    HttpResponse::Ok().json(response)
}

async fn default_not_found_handler() -> HttpResponse {
    HttpResponse::NotFound().json(serde_json::json!({"message": "not found"}))
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
