use postgres::{Client, NoTls};

fn main() {
    println!("Hello, Postgres!");
    string_stats();

    
    //conn = db_connect(stats_db_url)
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
         conn.execute(
            "INSERT INTO string_stats (uid) VALUES ($1)",
            &[&Uuid::new_v4().to_string()],
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
    println!("count 91: {}", uuid_vec.iter().filter(|&n| *n == 91.to_string()).count());

}
