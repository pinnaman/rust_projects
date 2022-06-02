use actix_web::Responder;
use std::error::Error;

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

    use chrono::prelude::*;
    use chrono::Duration;
    use plotters::prelude::*;

    // data for charts
    let data = crate::utils::get_fake_data();
    //println!("FAKE DATA=>{:?}",data);

    let data: Vec<(Date<Local>, f32, f32, f32, f32)> = data
        .iter()
        .map(|x| (crate::utils::timestamp_to_local_date(x.0), x.1, x.2, x.3, x.4))
        .collect();

        let dir = "/Users/apinnamaneni/rust_projects/db/api/src/plots-output";
        let filepath = format!("{}/sma15.png", &dir);
        let root = BitMapBackend::new(&filepath, (1280, 960)).into_drawing_area();
        root.fill(&WHITE).expect("Error filling background.");

         // Get date range
    let (end_date, start_date) = (
        data[0].0 + Duration::days(1),
        data[data.len() - 1].0 - Duration::days(1),
    );
    // Basic chart configuration
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .caption(
            "Candles + simple moving average",
            ("sans-serif", 50.0).into_font(),
        )
        .build_cartesian_2d(start_date..end_date, 0f32..300f32)
        .unwrap();
    // prepare chart layout
    chart
        .configure_mesh()
        .light_line_style(&WHITE)
        .draw()
        .unwrap();
    
    chart
    .draw_series(data.iter().map(|x| {
        CandleStick::new(
            x.0,
            x.1,
            x.2,
            x.3,
            x.4,
            RGBColor(98, 209, 61).filled(),
            RGBColor(209, 61, 61).filled(),
            10,
        )
    }))
    .unwrap();
    root.present().expect(&format!("Unable to write result to file please make sure directory '{}' exists under the current dir", &dir));
    println!("Plot has been saved to {}", &filepath);

    let close_price_data: Vec<f64> = data.iter().map(|x| x.4 as f64).collect();
    let sma_data = crate::utils::simple_moving_average(&close_price_data, 15).expect("Calculating SMA failed");
    let mut line_data: Vec<(Date<Local>, f32)> = Vec::new();
    for i in 0..sma_data.len() {
        line_data.push((data[i].0, sma_data[i] as f32));
    }

    chart
        .draw_series(LineSeries::new(line_data, BLUE.stroke_width(2)))
        .unwrap()
        .label("SMA 15")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));


    let sma_data = crate::utils::simple_moving_average(&close_price_data, 30).expect("Calculating SMA failed");
    let mut line_data: Vec<(Date<Local>, f32)> = Vec::new();
    for i in 0..sma_data.len() {
        line_data.push((data[i].0, sma_data[i] as f32));
    }

    chart
        .draw_series(LineSeries::new(
            line_data,
            RGBColor(150, 50, 168).stroke_width(2),
        ))
        .unwrap()
        .label("SMA 30")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(150, 50, 168)));

    chart
    .configure_series_labels()
    .position(SeriesLabelPosition::UpperMiddle)
    .label_font(("sans-serif", 30.0).into_font())
    .background_style(WHITE.filled())
    .draw()
    .unwrap();

    //format!("Chart this DATA=>{:?}",data)
    format!("#********DATA CHARTS*********#")

}