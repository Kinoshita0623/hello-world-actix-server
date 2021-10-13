#[macro_use]
extern crate diesel;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::pg::PgConnection;

mod repositories;
mod dao;
mod models;
mod schema;
use crate::repositories::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
mod errors;
mod service;
use crate::service::UserService;
mod state;
use crate::state::AppState;
use crate::service::RegisterUser;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let url = "postgres://dbuser:secret@db:5432/database";
    
    let manager = ConnectionManager::<PgConnection>::new(url);
    let pool: Pool<ConnectionManager<PgConnection>> = Pool::builder().max_size(20).build(manager).expect("Poolの作成に失敗");
  
    
    let app_state = AppState {
        pool: Box::new(pool),
    };
    //let arc = Arc::new(app_state);
    let data = web::Data::new(app_state);
    
    HttpServer::new( move || {
        App::new()
            .app_data(data.clone())
            .service(index)
            .service(echo)
            .service(register)
            .service(users)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}

#[get("/")]
async fn index<'a>(app_state: web::Data<AppState>) -> String {
    let repo = &app_state.user_repository();
    return repo.message();
    
}

#[get("/hello")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok().body("Hello world!");
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    return HttpResponse::Ok().body(req_body);
}

#[post("/register")]
async fn register(app_state: web::Data<AppState>, json: web::Json<RegisterUser>) -> impl Responder {
    match &app_state.user_service().register(json.0) {
        Ok(res) => {
            return HttpResponse::Ok().json(res);
        }
        Err(err) => {
            return err.response();
        }
    }
  
}

#[get("/users")]
async fn users(app_state: web::Data<AppState>) -> impl Responder {
    match &app_state.user_repository().find_all() {
        Ok(res) => {
            let simple_users: Vec<service::SimpleUser> = res.iter().map(|user|{
                return service::SimpleUser {
                    username: user.username.clone(),
                    id: user.id.clone()
                }
            }).collect();
            return HttpResponse::Ok().json(simple_users);
        }
        Err(_) => {
            return HttpResponse::Ok().body("Hey there!");  
        }
    }
}

async fn manual_hello() -> impl Responder {
    return HttpResponse::Ok().body("Hey there!");
}