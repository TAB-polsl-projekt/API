use rocket::Route;
use rocket_okapi::{okapi::{merge::marge_spec_list, openapi3::OpenApi}, settings::OpenApiSettings};

pub mod login;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
    // Start with an empty vector for routes and an initial empty OpenAPI object.
    let mut all_routes_and_docs = Vec::new();
    let prefix = "".to_string();

    all_routes_and_docs.push(login::get_routes_and_docs(settings));

    let (all_routes, all_spec) = all_routes_and_docs.into_iter()
        .fold((Vec::new(), Vec::new()), |(mut routes, mut specs), it| {
            routes.extend(it.0); specs.push(it.1); (routes, specs)
        });

    let spec_list: Vec<(&String, OpenApi)> = all_spec.into_iter().map(|x| (&prefix, x)).collect();
    let merged_spec = marge_spec_list(&spec_list).unwrap();

    (all_routes, merged_spec)
}
