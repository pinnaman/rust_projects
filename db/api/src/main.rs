use postgres::{Client, NoTls, Error};
use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Responder};
use dotenv::dotenv;
use std::env;
//use chrono::prelude::Utc;

// mod db_pg;
mod handlers;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    println!("#*****Hello, API world!*********#");

    // data for charts
    //let data = utils::get_fake_data();
    //println!("FAKE DATA=>{:?}",data);
    // Convert timestamp to Date<Local>
    /*let data: Vec<(Date<utils::Local>, f32, f32, f32, f32)> = data
        .iter()
        .map(|x| (utils::timestamp_to_local_date(x.0), x.1, x.2, x.3, x.4))
        .collect();
    */

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
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}