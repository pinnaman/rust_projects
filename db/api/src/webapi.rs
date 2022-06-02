use actix_web::Responder;
use std::error::Error;
use std::time::Duration;
use serde_json::json;
use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Move {
    name: String,
}
#[derive(Debug, Deserialize)]
struct PokemonMove {
    #[serde(rename="move")]
    move_: Move,
}
#[derive(Debug, Deserialize)]
struct Pokemon {
    id: i32,
    name: String,
    height: i32,
    weight: i32,
    moves: Vec<PokemonMove>,
}

pub async fn ipinfo() -> impl Responder {

   /*  let resp = reqwest::blocking::get("https://httpbin.org/ip")
    .unwrap()
    .json::<HashMap<String, String>>()
    .unwrap();
    println!("{:#?}", resp);
    //return "ok";
*/
    format!("Pokemon Data")

}

//pub async fn get_data() -> Result<(), Box<dyn Error>> {
pub async fn get_data() -> impl Responder {

    println!("#********** SCRAPER*******#");
    let args = vec!["arg"];
    let arg = args.get(1).unwrap_or(&"AAPL");
    let b = reqwest::get("https://swapi.dev/api/people").await;
    let joke = reqwest::get("https://v2.jokeapi.dev/joke/Any?safe-mode").await;
    //Ok(json!(joke));
    println!("{:?}", b);
    println!("JOKE=>{:?}", joke);
    format!("{}", arg)

}