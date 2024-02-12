use actix_web::HttpResponse;

pub async fn home() -> HttpResponse {
    return HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("home.html"));
}
