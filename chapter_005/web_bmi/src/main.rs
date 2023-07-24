use actix_web::{web, App, HttpResponse, HttpRequest, HttpServer, Error};
use serde::{Deserialize, Serialize};

const SERVER_ADDR: &str = "127.0.0.1:8888";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is running on http://{}", SERVER_ADDR);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/bmi", web::get().to(bmi))
    })
    .bind(SERVER_ADDR)?
    .run()
    .await
}

async fn index(_: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FormBMI {
    height: f64,
    weight: f64,
}

async fn bmi(form: web::Query<FormBMI>) -> Result<HttpResponse, Error> {
    println!("{:?}", form);
    let h = form.height / 100.0;
    let bmi = form.weight / (h * h);
    let per = bmi / 22.0 * 100.0;

    let result = if bmi < 18.5 {
        format!("体型: やせ型")
    } else if bmi < 25.0 {
        format!("体型: 標準")
    } else if bmi < 30.0 {
        format!("体型: 肥満(1度)")
    } else if bmi < 35.0 {
        format!("体型: 肥満(2度)")
    } else if bmi < 40.0 {
        format!("体型: 肥満(3度)")
    } else {
        format!("体型: 肥満(4度)")
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        // .content_type("application/x-www-form-urlencoded; charset=utf-8")
        .body(format!(
            "<h3>あなたのBMIは{:.2}です。乖離率: {:.2}%です。{}</h3>",
            bmi, per, result
        )))
}
