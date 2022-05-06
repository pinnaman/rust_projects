use postgres::{Client, NoTls, Error};
//use chrono::prelude::Utc;

fn main() {
    println!("Hello, Postgres!");
    //string_stats();
    let res = pg_main();
    match res {
        Ok(_) => {println!("DB Succeeded!");},
        Err(e) => {println!("Error: {}!", e);}
    }
    
}

#[allow(dead_code,warnings, unused_variables, unused_assignments)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[allow(dead_code,warnings, unused_variables, unused_assignments)]
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

fn pg_main() -> Result<(), Error> {
    use regex::Regex;
    //let stats_db_url = "postgresql://postgres:postgres@localhost:5432/stats_db";
    let news_db_url = "postgresql://postgres:postgres@localhost:5432/news_db";
    let mut client= Client::connect(news_db_url, NoTls)?;

    println!("drop and create users and articles tables");
    client.batch_execute("drop table articles; drop table users;")?;
    /*
    CREATE TABLE string_stats (
        index serial,
        uid text,
        number_cnt INTEGER,
        alpha_cnt INTEGER NOT NULL,
        vowel_cnt INTEGER,
        ins_date TIMESTAMP NOT NULL DEFAULT NOW()
      );
      */
    client.batch_execute("\
        CREATE TABLE users (
            id    SERIAL PRIMARY KEY,
            name  TEXT NOT NULL,
            email TEXT NOT NULL,
            age   INTEGER NOT NULL,
            ins_date TIMESTAMP NOT NULL DEFAULT NOW()
        )
    ")?;
    ////TIMESTAMP WITH TIME ZONE NOT NULL,
    client.batch_execute("\
        CREATE TABLE articles (
            id             SERIAL PRIMARY KEY,
            title          TEXT NOT NULL,
            body           TEXT NOT NULL,
            published_at   TIMESTAMP NOT NULL DEFAULT NOW() ,
            author_id      INTEGER REFERENCES users(id)
        )
    ")?;

    let name = "James";
    let email = "james@test.com";
    let age:i32 = 26;
    client.execute(
        "INSERT INTO users (name, email, age) VALUES ($1, $2, $3)",
        &[&name, &email, &age],
    )?;

    for row in client.query("SELECT * FROM users", &[])? {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let email: &str = row.get(2);
        let age: i32 = row.get(3);

        println!("Found person: {} {} {} {}", id, name, email, age);

        let title: &str = "A great article!";
        let body: &str = "What should you be reading about?";
        //let cur_time = Utc::now();
        client.execute(
            "INSERT INTO articles (title, body,  author_id) VALUES ($1, $2, $3)",
            &[&title, &body, &id]
        )?;
    }

    use uuid::Uuid;
    println!("#******STRING STATS************#");
    let vowels = Regex::new(r"[aeiouy]").unwrap();
    for _ in 0..10 {
        //println!("{}", Uuid::new_v4());
        //uuid_vec.push(Uuid::new_v4().to_string());
        let uid = Uuid::new_v4().to_string();
        //let str_length = &uid.chars().count();
        let z_cnt = &uid.chars().filter(|c| vowels.is_match(&c.to_string())).count();
        let al_cnt = &uid.chars().filter(|c| c.is_alphabetic()).count();
        let num_cnt = &uid.chars().filter(|c| c.is_numeric()).count();
        //println!("Count {},{},{},{}",uid,num_cnt,al_cnt,z_cnt);
        let z_32: i32 = *z_cnt as i32;
        let al_32: i32 = *al_cnt as i32;
        let num_32: i32 = *num_cnt as i32;
        //print_type_of(&num_32); 
        
       client.execute(
        "INSERT INTO string_stats (uid,vowel_cnt,alpha_cnt,number_cnt) VALUES ($1,$2,$3,$4)",
        &[&uid,&z_32,&al_32,&num_32],
        //&[&uid],
    )?;
    
    }

    struct User {
        name: String,email: String,age: i32
    }
    for row in &client.query("SELECT name,email,age FROM users", &[]).unwrap() {
        let user = User {name: row.get(0),email: row.get(1),age: row.get(2),};
        println!("Found student {} with email:{} aged=> {}", user.name, user.email, user.age);
    }
    return Ok(());
}