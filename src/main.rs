use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;

struct AppState {
    pub app_name: String
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| {
        let mut rnd = rand::thread_rng();
        let n: usize = rnd.gen();
        App::new()
            .data(AppState {
                app_name: String::from(format!("Actix-web number:{}", n)),
            })
            .service(index)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    return format!("Hello {}", app_name);
}

#[get("/hello")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok().body("Hello world!");
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    return HttpResponse::Ok().body(req_body);
}

async fn manual_hello() -> impl Responder {
    return HttpResponse::Ok().body("Hey there!");
}