use actix_web::{ web, App, HttpServer };
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{ lmdb::utils::init_db, routes::order::order_config, utopia::openapi::ApiDoc };
mod scripts;
mod lmdb;
mod utopia;
mod schema;
mod routes;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let path = "./lmdb_data";
    let db = init_db(path).await.expect("Failed to initialize database");

    println!("🚀 Server starting at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(order_config) // routes
            .service(
                SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi())
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run().await
}
