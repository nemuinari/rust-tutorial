use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("An error occurred")]
    InternalError,

    #[error("Not found: {0}")]
    NotFound(String),
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MyError::InternalError => HttpResponse::InternalServerError().finish(),
            MyError::NotFound(_) => HttpResponse::NotFound().finish(),
        }
    }
}

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let response_body = "Hello, world!";
    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
// http://localhost:8080/
