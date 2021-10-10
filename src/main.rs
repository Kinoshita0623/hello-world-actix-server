#[macro_use]
extern crate diesel;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
use diesel::pg::PgConnection;

use diesel::prelude::*;
mod repositories;
mod dao;
mod models;
use crate::dao::*;
mod schema;
use crate::repositories::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use serde::Deserialize;
use std::sync::Arc;
use crate::models::NewUser;
use crate::models::User;

#[derive(Clone)]
struct AppState<'a> {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>,
    pub hoge: &'a str,
}

impl<'a> AppState<'a> {
    pub fn user_repository(&self) -> impl UserRepository {
        
        
        return UserDAO {
            pool: self.pool.clone()
        }
    }
}

#[derive(Deserialize)]
struct Register {
    username: String,
    password: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let url = "postgres://dbuser:secret@db:5432/database";
    
    let manager = ConnectionManager::<PgConnection>::new(url);
    let pool: Pool<ConnectionManager<PgConnection>> = Pool::builder().max_size(20).build(manager).expect("Poolの作成に失敗");
  
    
    let app_state = AppState {
        pool: Box::new(pool),
        hoge: "hogehoge"
    };
    //let arc = Arc::new(app_state);
    let data = web::Data::new(app_state);
    
    HttpServer::new( move || {
        App::new()
            .app_data(data.clone())
            .service(index)
            .service(echo)
            .service(register)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}

#[get("/")]
async fn index<'a>(app_state: web::Data<AppState<'a>>) -> String {
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
async fn register<'a>(app_state: web::Data<AppState<'a>>, json: web::Json<Register>) -> impl Responder {
    let repo = &app_state.user_repository();
    let res = repo.create(NewUser{
        username: &json.username,
        encrypted_password: &json.password
    });
    match res {
        Ok(res) => {
            return HttpResponse::Ok().body(format!("id: {}, username:{}", res.id, res.username));
        }
        Err(err) => {
            return HttpResponse::Ok().body(format!("error:{}", err));
        }
    }
}
async fn manual_hello() -> impl Responder {
    return HttpResponse::Ok().body("Hey there!");
}