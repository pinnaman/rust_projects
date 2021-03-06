#![allow(dead_code, unused)]

use postgres::{Client, NoTls, Error};
use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Responder};
use actix_files as fs;
use dotenv::dotenv;
use std::env;

// mod db_pg;
mod handlers;
mod utils;
mod webapi;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    println!("#*****Hello, API world!*********#");

    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug");
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");
    let db_url = env::var("DATABASE_URL").expect("URL not set");

    // Start http server
    HttpServer::new(move || {
        App::new()
           
            .route("/users", web::get().to(handlers::get_users))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users", web::post().to(handlers::add_user))
            .route("/users/{id}", web::delete().to(handlers::delete_user))
            .route("/stats", web::get().to(handlers::string_stats))
            .route("/nstats", web::get().to(handlers::num_stats))
            .route("/charts", web::get().to(handlers::charts))
            .route("/scrape", web::get().to(webapi::get_data))
            .route("/ip", web::get().to(webapi::ipinfo))
             //.service(fs::Files::new("/images", "./plots-output").prefer_utf8(true))
            .service(fs::Files::new("/", "./").show_files_listing().index_file("index.html"))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}