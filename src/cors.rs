use actix_cors::Cors;
use actix_web::http;

/// Create the default CORS
pub fn new(origin: &str) -> Cors { 
    Cors::default()
        .allowed_origin(origin)
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}
