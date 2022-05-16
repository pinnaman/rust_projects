// src/handlers.rs

use actix_web::{web, HttpRequest, Responder};
//extern crate dotenv;

pub async fn get_users() -> impl Responder {
    format!("hello from get users")
}

pub async fn get_user_by_id() -> impl Responder {
    format!("hello from get users by id")
}

pub async fn add_user() -> impl Responder {
    format!("hello from add user")
}

pub async fn delete_user() -> impl Responder {
    format!("hello from delete user")
}

pub async fn string_stats() -> impl Responder {
    use uuid::Uuid;
    let mut uuid_vec: Vec<String> = Vec::new();
    for _ in 0..10 {
        //println!("{}", Uuid::new_v4());
        uuid_vec.push(Uuid::new_v4().to_string());
    }
    //println!("{:?}",uuid_vec);
    format!("{:?}",uuid_vec);
    
}


pub async fn num_stats() -> impl Responder {

    //println!("#********** NUMBER STATS*******#");

    use rand::{distributions::Uniform, distributions::Standard,Rng}; // 0.6.5
    
    let mut rng = rand::thread_rng();
    let i:i32 = rng.gen_range(0,20);
    let f:f32 = rng.gen_range(0.0,10.0);
    //println!("Random Integer=> {}", i);
    //println!("Random Float=> {}", f);

    //let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 20);
    let vals_int1: Vec<u64> = (0..10).map(|_| rng.sample(&range)).collect();
    let vals_int2: Vec<u64> = (0..10).map(|_| rng.sample(&range)).collect();
    //println!("Int Arr1=>{:?},Int Arr2=>{:?}", vals_int1, vals_int2);
    format!("Int Arr1=>{:?},Int Arr2=>{:?}", vals_int1, vals_int2);

    //vector of random values witin a range
    let range = Uniform::from(0.0..10.0);
    let values: Vec<f64> = rand::thread_rng().sample_iter(&range).take(10).collect();
    //println!("Uniform Float < 10=>{:?}", values);

    //vector of random values without limiting to a range
    let values: Vec<u8> = rand::thread_rng().sample_iter(Standard).take(10).collect();
    format!("Random Values=>{:?}", values);
   
}