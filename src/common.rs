use std::any::Any;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct JsonResponse<T: Any> {
    success: bool,
    status: i32,
    msg: String,
    data: T,
    total: i64,
}

impl<T: Any> JsonResponse<T> {
    pub fn build(data: T) -> JsonResponse<T> {
        JsonResponse { success: true, status: 200, msg: String::from("success"), data, total: 1 }
    }

    pub fn ok() -> JsonResponse<dyn Any> {
        JsonResponse::build(None)
    }

    pub fn error(msg: String, status: i32) -> JsonResponse<dyn Any> {
        JsonResponse { success: false, status, msg, data: None, total: 1 }
    }
}
