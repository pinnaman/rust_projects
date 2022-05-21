// src/handlers.rs

use actix_web::Responder;

//use utils::*;

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
    use regex::Regex;
    let mut uuid_vec: Vec<String> = Vec::new();
    let vowels = Regex::new(r"[aeiouy]").unwrap();
    for _ in 0..10 {
        //println!("{}", Uuid::new_v4());
        let uid = Uuid::new_v4().to_string();
        //uuid_vec.push(Uuid::new_v4().to_string());
        uuid_vec.push(uid);
        
    }
    for (pos, e) in uuid_vec.iter().enumerate() {
        //println!("{}: {:?}", pos, e);
        let z_cnt = e.chars().filter(|c| vowels.is_match(&c.to_string())).count();
        let al_cnt = e.chars().filter(|c| c.is_alphabetic()).count();
        let num_cnt = e.chars().filter(|c| c.is_numeric()).count();
        println!("Count=>{},{},{},{}",e,num_cnt,al_cnt,z_cnt);
    }
    //println!("{:?}",uuid_vec);
    format!("{:?}",uuid_vec);
    format!("hello from string stats=>{:?}",uuid_vec)
}
    
pub async fn num_stats() -> impl Responder {

    println!("#********** NUMBER STATS*******#");

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
    println!("Int Arr1=>{:?},Int Arr2=>{:?}", vals_int1, vals_int2);
    //format!("Int Arr1=>{:?},Int Arr2=>{:?}", vals_int1, vals_int2);

    //vector of random values witin a range
    let range = Uniform::from(90.0..100.0);
    let values: Vec<f64> = rand::thread_rng().sample_iter(&range).take(10).collect();
    println!("Uniform Float 90-100=>{:?}", values);

    //vector of random values without limiting to a range
    let values: Vec<u8> = rand::thread_rng().sample_iter(Standard).take(10).collect();
    //format!("Random Values=>{:?}", values);

    format!("Random NUmbers=>{:?}",values)
   
}


pub async fn charts() -> impl Responder {

    // data for charts
    let data = crate::utils::get_fake_data();
    //println!("FAKE DATA=>{:?}",data);

    format!("Chart this DATA=>{:?}",data)

}