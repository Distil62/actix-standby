use actix_web::{
    dev::HttpResponseBuilder,
    error,
    http::{header, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", message)]
pub struct MyError {
    pub message: String,
    pub status_code: StatusCode,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code)
            .set_header(header::CONTENT_TYPE, "application/json; chartset=utf-8")
            .body(format!("{{ \"message\": \"{}\"}}", self.message))
    }
}
