use actix_web::{App, get, HttpServer, middleware, Responder, web};

mod api;
mod middle;
mod common;

#[get("/hi/{name}")]
async fn say_hi(name: web::Path<String>) -> impl Responder {
    common::JsonResponse::<bool>::ok().with_msg(name.into_inner())
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        let api_scope = web::scope("/api")
            .wrap(middleware::Logger::default())
            .wrap(middle::JwtAuth::default())
            .service(greet)
            .service(say_hi)
            .service(api::get_user)
            .service(api::auth)
            .service(api::refresh)
            ;

        let static_file_scope = actix_files::Files::new("/", "dist").index_file("index.html");

        App::new()
            // enable automatic response compression - usually register this first
            // .wrap(middleware::Compress::default())
            // 路径规范化，以及重定向到附加斜杠的路由
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::MergeOnly,
            ))
            // enable logger - always register Actix Web Logger middleware last
            .service(api_scope)
            .service(web::scope("")
                .wrap(middleware::Compress::default())
                .service(static_file_scope)
            )
    })
        .bind(("127.0.0.1", 8080))? // for IPv4
        .bind(("::1", 8080))? // for IPv6
        .workers(4) // 指定 worker 线程数量
        .run()
        .await
}
