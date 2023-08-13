use actix_web::{HttpRequest, HttpResponse, Responder, web};
use actix_web::body::EitherBody;
use serde::Serialize;

#[derive(Serialize)]
#[allow(non_snake_case, dead_code)]
pub struct JsonResponse<T: Serialize> {
    status: i32,
    success: bool,
    msg: String,
    data: Option<T>,
    total: Option<i64>,
}

impl<T: Serialize> Responder for JsonResponse<T> {
    type Body = EitherBody<String>;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        web::Json(self).respond_to(req)
    }
}

#[allow(dead_code)]
impl<T: Serialize> JsonResponse<T> {
    pub fn build(data: Option<T>) -> Self {
        JsonResponse { success: true, status: 200, msg: "success".to_string(), data, total: None }
    }

    pub fn with_msg(mut self, msg: String) -> Self {
        self.msg = msg;
        self
    }

    pub fn set_total(mut self, total: i64) -> Self {
        self.total = Some(total);
        self
    }
}

#[allow(dead_code)]
impl JsonResponse<bool> {
    pub fn ok() -> Self {
        JsonResponse::build(None)
    }

    pub fn error(msg: String, status: i32) -> Self {
        JsonResponse { success: false, status, msg, data: None, total: None }
    }
}
