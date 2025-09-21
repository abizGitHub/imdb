use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::derive::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum MyError {
    #[display("internal error")]
    InternalError,

    #[display("wrong file: {file_name}")]
    InvalidFileName { file_name: String },

    #[display("actor not found: {primary_name}")]
    ActorNameNotFound { primary_name: String },

    #[display("genre not found: {genre}")]
    GenreNotFound { genre: String },
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::InvalidFileName { .. } => StatusCode::BAD_REQUEST,
            MyError::ActorNameNotFound { .. } => StatusCode::NOT_FOUND,
            MyError::GenreNotFound { .. } => StatusCode::NOT_FOUND,
        }
    }
}
