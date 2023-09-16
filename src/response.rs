use actix_web::{HttpResponse, http::header};
use serde_derive::Serialize;
use serde::ser;

#[derive(Serialize)]
pub struct JsonOK {
    pub code: u32,
}

#[derive(Serialize)]
pub struct JsonError<T: AsRef<str>> { 
    pub code: u32,
    pub message: T,
}

#[derive(Serialize)]
pub struct JsonResult<T: ser::Serialize> { 
    pub code: u32,
    pub result: T,
}

#[derive(Serialize)]
pub struct JsonAuth { 
    pub authorization: String,
}

/// response 200
pub fn ok() -> HttpResponse { 
    HttpResponse::Ok().json(JsonOK{code: 0})
}

/// access denied
pub fn deny() -> HttpResponse { 
    HttpResponse::Ok().body("deny")
}

/// response error message
pub fn error<T: AsRef<str>>(message: T) -> HttpResponse { 
    let err = JsonError { 
        code: 500,
        message: message.as_ref(),
    };
    let result = serde_json::to_string(&err).unwrap();
    HttpResponse::Ok().content_type("application/json").body(result)
}

/// response complicated data
pub fn result<T: ser::Serialize>(result: &T) -> HttpResponse { 
    let res = JsonResult { 
        code: 0,
        result: result,
    };
    HttpResponse::Ok().json(res)
}

/// Page redirection
pub fn redirect(url: &str) -> HttpResponse { 
    HttpResponse::Found().append_header((header::LOCATION, url)).finish()   
}
