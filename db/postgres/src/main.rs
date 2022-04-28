use postgres::{Client, NoTls};

fn main() {
    println!("Hello, Postgres!");
    string_stats();
    dbconn();
    
}
fn dbconn() {

    struct User {
        username: String,
        password: String
    }

    println!("DB Connection!");
    let stats_db_url = "postgresql://postgres:postgres@localhost:5432/stats_db";
    let mut conn = Client::connect(stats_db_url, NoTls).unwrap();

    /*conn.execute(
        "INSERT INTO app_user (username, password, email) VALUES ($1, $2, $3)",
        &[&"user3", &"mypass3", &"user3@test.com"],
    ).unwrap();
    */

    for row in &conn.query("SELECT username, password FROM app_user", &[]).unwrap() {
        let user = User {
            username: row.get(0),
            password: row.get(1)
        };
        println!("Found student {} with ID:{}", user.username, user.password);
    }

    use uuid::Uuid;
     for _ in 0..10 {
         //println!("{}", Uuid::new_v4());
         //uuid_vec.push(Uuid::new_v4().to_string());
         let uid = Uuid::new_v4().to_string();
         //let str_length = &uid.chars().count();
         let z_cnt = &uid.chars().filter(|c| *c == 'a').count();
         let al_cnt = &uid.chars().filter(|c| c.is_alphabetic()).count();
         let num_cnt = &uid.chars().filter(|c| c.is_numeric()).count();
         println!("Count {},{},{},{}",uid,num_cnt,al_cnt,z_cnt);
         
         conn.execute(
            "INSERT INTO string_stats (uid) VALUES ($1)",
            //&[&uid,&num_cnt,&al_cnt],
            &[&uid],
        ).unwrap();
        
     }

}
fn string_stats() {

    use uuid::Uuid;
    let mut uuid_vec: Vec<String> = Vec::new();
    for _ in 0..10 {
        //println!("{}", Uuid::new_v4());
        uuid_vec.push(Uuid::new_v4().to_string());
    }
    println!("{:?}",uuid_vec);
    println!("count 9: {}", uuid_vec.iter().filter(|&n| *n == 9.to_string()).count());

}
