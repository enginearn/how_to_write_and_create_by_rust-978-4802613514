use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

const SERVER_ADDRESS: &str = "127.0.0.1:8080";

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/hello/{name}")]
async fn hello_name(path: web::Path<(String,)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", path.0))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello)
        .service(echo)
        .service(hello_name)
        .route("/hey", web::get().to(manual_hello))
    })
    .bind(SERVER_ADDRESS)?
    .run()
    .await
}
