use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;

pub fn e500<T>(err: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    return actix_web::error::ErrorInternalServerError(err);
}

pub fn see_other(location: &str) -> HttpResponse {
    return HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish();
}

pub fn e400<T>(err: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    return actix_web::error::ErrorBadRequest(err);
}
