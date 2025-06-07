use db::DbConn;
use diesel::SqliteConnection;
use rocket::{catch, catchers, http::Method, launch, options, response::status, Request};
use rocket_cors::{AllowedOrigins, CorsOptions};
mod api;
pub mod session;

use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use rocket_okapi::mount_endpoints_and_merged_docs;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use std::env;

use crate::db::DbPool;

pub mod dbschema;
pub mod dbmodels;
pub mod db;

#[catch(404)]
fn not_found(req: &Request) -> status::NotFound<String> {
    let message = format!("The route '{}' was not found.", req.uri());
    status::NotFound(message)
}

#[options("/<_..>")]
fn options_preflight() {}

fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}


#[launch]
async fn rocket() -> _ {

    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Post, Method::Options]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .to_cors()
    .expect("CORS configuration failed");

    let mut rocket_builder = rocket::build()
    .attach(DbConn::fairing())
        .attach(cors)
        .mount("/swagger-ui/", make_swagger_ui(&SwaggerUIConfig {
            url: "../api/openapi.json".to_owned(),
            ..Default::default()
        }))
        .register("/", catchers![not_found]);

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
        mount_endpoints_and_merged_docs! {
            rocket_builder, "/api".to_owned(), openapi_settings,
            "/" => api::get_routes_and_docs(&openapi_settings)
        };

    rocket_builder
}
