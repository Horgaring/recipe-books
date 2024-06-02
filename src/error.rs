use std::io::Cursor;
use rocket::http::{ Header, Status};
use rocket::{Request, response, Response};
use rocket::response::{ Responder};
use serde::{ Serialize};

#[derive( Debug, Clone)]
pub enum CustomError {
    //#[resp("{0}")]


    //#[resp("{0}")]
    NotFound(String),

    //#[resp("{0}")]
    BadRequest(String),

    Redirect(String),
    Unauthorized,
    PermissionDenied(&'static str),
}
impl CustomError {
    fn get_http_status(&self) -> Status {
        match self {
            CustomError::Redirect(_) => Status::SeeOther,
            CustomError::NotFound(_) => Status::NotFound,
            CustomError::BadRequest(_) => Status::BadRequest,
            CustomError::Unauthorized => Status::Unauthorized,
            CustomError::PermissionDenied(_) => Status::new(403),
        }
    }
    fn get_header(&self) -> Header<'static> {
        match self {
            CustomError::Redirect(path) => Header::new("Location",path.clone()),
            CustomError::PermissionDenied(_) => Header::new("Content-Type","application/json"),
            CustomError::NotFound(_) => Header::new("Content-Type","application/json"),
            CustomError::BadRequest(_) => Header::new("Content-Type","application/json"),
            CustomError::Unauthorized => {Header::new("Content-Type","application/json")}
        }
    }
}
impl std::fmt::Display for CustomError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Error {}.", self.get_http_status())
    }
}
#[derive(Serialize)]
struct ErrorResponse {
    message: String
}

impl<'r> Responder<'r, 'static> for CustomError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let mes = match &self {
            CustomError::NotFound(path) => {path}
            CustomError::BadRequest(mass) => {mass}
            CustomError::Redirect(mass) => {mass}
            CustomError::Unauthorized => {"401 Unauthorized"}
            CustomError::PermissionDenied(mess) => mess,
        };
        // serialize struct into json string
        let err_response = serde_json::to_string(&ErrorResponse{
            message: mes.to_string()
        }).unwrap();

        Response::build()
            .status(self.get_http_status())
            .header(self.get_header())
            .sized_body(err_response.len(), Cursor::new(err_response))
            .ok()
    }
}

