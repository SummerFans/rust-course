use actix_web::{get, web, App, HttpRequest, HttpServer, HttpResponse, Responder, guard};
use std::sync::Mutex;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[get("/code/{name}")]
async fn code(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {} counter:{}!", &name, counter)
}

async fn goods_item(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {} counter:{}!", &name, counter)
}

struct AppState {
    app_name: String,
    counter: Mutex<i32>
}

#[get("/index")]
async fn index(req: HttpRequest, data: web::Data<AppState>) -> String {
    let header = req.headers();
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    match header.get(app_name) {
        Some(v) => {
            format!("{:?}; counter: {}", v, counter)
        }
        None => {
            format!("app_name: is null")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_state = web::Data::new(AppState {
        app_name: String::from("X-client-id"),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(web::scope("/goods").route("/{name}", web::get().to(goods_item)))
            .service(code)
            .service(index)
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "www.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("www"))),
            )
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "users.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("user"))),
            )
            .route("/", web::to(|| HttpResponse::Ok()))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}