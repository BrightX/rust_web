// use std::any::Any;
use actix_web::{get, Responder, web};
// use bcrypt::{hash, verify};
use chrono::{DateTime, Local};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

// use crate::common::JsonResponse;

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct User {
    username: String,
    age: i32,
    createTime: DateTime<Local>,
    password: Option<String>,
    ver: bool,
}

#[get("/user")]
pub async fn get_user() -> web::Json<User> {
    // let passwd = hash("admin123", 10);
    // let ver = verify("123456", "$2a$10$AHbCM2O/BdscPmgTXdW5.OzSEBvaxBLKQLFklQkFextV8EsoTplMC");
    web::Json(User {
        username: "张三".parse().unwrap(),
        age: 18,
        createTime: Local::now(),
        // password: Option::from(passwd.unwrap()),
        password: None,
        // ver: ver.unwrap(),
        ver: true,
    })
}

#[derive(Deserialize)]
#[allow(non_snake_case, dead_code)]
pub struct AuthQuery {
    username: String,
    password: String,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct AuthToken {
    access_token: Option<String>,
    refresh_token: Option<String>,
    msg: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AccessToken {
    uid: Option<String>,
    username: Option<String>,
    exp: i64,
}

#[get("/auth")]
pub async fn auth(auth: web::Query<AuthQuery>) -> web::Json<AuthToken> {
    let head: Header = Header::new(Algorithm::HS512);
    let at_encode_key: EncodingKey = EncodingKey::from_secret("my_secret_access_token".as_ref());
    let rt_encode_key: EncodingKey = EncodingKey::from_secret("my_secret_refresh_token".as_ref());

    let a_uid = String::from("a1");
    let r_uid = String::from("a1");
    let at = AccessToken {
        uid: Option::from(a_uid),
        username: Option::from(String::from(&auth.username)),
        exp: Local::now().timestamp() + 3600 * 2,
    };
    let rt = AccessToken {
        uid: Option::from(r_uid),
        username: Option::from(String::from(&auth.username)),
        exp: Local::now().timestamp() + 3600 * 24 * 14,
    };
    return web::Json(AuthToken {
        access_token: Option::from(jsonwebtoken::encode(&head, &at, &at_encode_key).unwrap()),
        refresh_token: Option::from(jsonwebtoken::encode(&head, &rt, &rt_encode_key).unwrap()),
        msg: None,
    });
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct RefreshQuery {
    refresh_token: String,
}

#[get("/refresh")]
pub async fn refresh(query: web::Query<RefreshQuery>) -> impl Responder {
    let head: Header = Header::new(Algorithm::HS512);
    let at_encode_key: EncodingKey = EncodingKey::from_secret("my_secret_access_token".as_ref());
    let rt_encode_key: EncodingKey = EncodingKey::from_secret("my_secret_refresh_token".as_ref());
    let rt_decode_key: DecodingKey = DecodingKey::from_secret("my_secret_refresh_token".as_ref());
    let validation = Validation::new(Algorithm::HS512);

    let result = jsonwebtoken::decode::<AccessToken>(&query.refresh_token, &rt_decode_key, &validation);
    if result.is_err() {
        return web::Json(AuthToken {
            access_token: None,
            refresh_token: None,
            msg: Option::from(result.err().unwrap().to_string()),
        });
    }

    let claims = result.unwrap().claims;
    let at = AccessToken {
        uid: Option::from(claims.uid.clone()),
        username: Option::from(claims.username.clone()),
        exp: Local::now().timestamp() + 3600 * 2,
    };
    let at = jsonwebtoken::encode(&head, &at, &at_encode_key).unwrap();

    let rt = if claims.exp < Local::now().timestamp() - 3600 * 6 {
        let rt = AccessToken {
            uid: Option::from(claims.uid.clone()),
            username: Option::from(claims.username.clone()),
            exp: Local::now().timestamp() + 3600 * 24 * 14,
        };
        jsonwebtoken::encode(&head, &rt, &rt_encode_key).unwrap()
    } else {
        query.refresh_token.clone()
    };

    web::Json(AuthToken {
        access_token: Option::from(at),
        refresh_token: Option::from(rt),
        msg: None,
    })
}

// #[get("/ok")]
// pub async fn get_ok() -> JsonResponse<dyn Any> {
//     JsonResponse::ok()
// }
