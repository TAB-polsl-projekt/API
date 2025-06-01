use std::env;

use rocket_okapi::{openapi_get_spec, settings::OpenApiSettings};
use rocket_okapi::okapi::openapi3::OpenApi;
use serde_json;

use qnl_worker::api;

pub fn write_openapi_file(path: &str) -> std::io::Result<()> {
    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    let (_, spec) = api::get_routes_and_docs(&openapi_settings);
    let json = serde_json::to_string_pretty(&spec)
        .expect("failed to serialize OpenAPI spec");
    std::fs::write(path, json)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Specify only <output file>");
        return;
    }

    write_openapi_file(args[1].as_str()).unwrap();
}