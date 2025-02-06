use actix_web::HttpResponse;

pub(crate) fn add_peers_handler(_peers: Vec<String>) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().finish())
}

pub(crate) fn get_peers_handler() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().json(vec!["127.0.0.1".to_string()]))
}
