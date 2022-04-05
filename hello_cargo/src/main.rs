///extern crate rand;

fn main() {

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds=> {}", THREE_HOURS_IN_SECONDS);

    functional_patterns();
    string_stats();

    //num_stats();

    //matrix();

    //class();
    
    /*
    shadowing();

    dtypes();

    cli()
    */
    
}

#[allow(dead_code,warnings, unused_variables, unused_assignments)]
fn functional_patterns() {

    let psum: u64 = vec![1, 2, 3].into_iter().map(|x| x*x*x).sum();
    println!("{:?}",psum);

    let v = vec![-1, 2, -3, 4, 5].into_iter();
    let _positive_numbers: Vec<i32> = v
        .inspect(|x| println!("Before filter: {}", x))
        .filter(|x: &i32| x.is_positive())
        .inspect(|x| println!("After filter: {}", x))
        .collect();
}

#[allow(dead_code,warnings, unused_variables, unused_assignments)]
fn string_stats() {

    use uuid::Uuid;
     // uuid vector 
     let mut uuid_vec: Vec<String> = Vec::new();
     for _ in (0..10) {
         //println!("{}", Uuid::new_v4());
         uuid_vec.push(Uuid::new_v4().to_string());
     }
     println!("{:?}",uuid_vec);
     //for value in uuid_vec {
     //    println!("ITER: {}", value);
     //}
     println!("count 91: {}", uuid_vec.iter().filter(|&n| *n == 91.to_string()).count());

}

#[allow(dead_code,warnings, unused_variables, unused_assignments)]
fn num_stats() {

    use rand::{distributions::Uniform, distributions::Standard,Rng}; // 0.6.5
    
    let mut rng = rand::thread_rng();
    let i:i32 = rng.gen_range(0,10);
    let f:f32 = rng.gen_range(0.0,10.0);
    println!("Integer: {}", i);
    println!("Float: {}", f);

    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 20);
    let vals_int1: Vec<u64> = (0..10).map(|_| rng.sample(&range)).collect();
    let vals_int2: Vec<u64> = (0..10).map(|_| rng.sample(&range)).collect();
    println!("Int Arr1=>{:?},Int Arr2=>{:?}", vals_int1, vals_int2);

    //vector of random values witin a range
    let range = Uniform::from(0.0..10.0);
    let values: Vec<f64> = rand::thread_rng().sample_iter(&range).take(10).collect();
    println!("Uniform Float < 10=>{:?}", values);

    //vector of random values without limiting to a range
    let values: Vec<u8> = rand::thread_rng().sample_iter(Standard).take(10).collect();
    println!("{:?}", values);

   

}

#[allow(dead_code,warnings, unused_variables, unused_assignments)]
fn matrix() {

    //2x2
    let mut x = vec![vec![0.0f64; 2]; 2];
    println!("{:?}",x);
    //3x3
    let y = [[[0.0f64; 3]; 3]; 3];
    println!("{:?}",y);
}

#[allow(dead_code,warnings, unused_variables, unused_assignments)]
fn class() {

    //use std::fmt;
    #[derive(Debug)]
    struct Person {
        first_name: String,
        last_name: String
    }
    impl Person {
        fn new(first: &str, name: &str) -> Person {
            Person {
                first_name: first.to_string(),
                last_name: name.to_string()
            }
        }
        fn full_name(&self) -> String {
            format!("{} {}",self.first_name, self.last_name)
        }
        fn set_first_name(&mut self, name: &str) {
            self.first_name = name.to_string();
        }
        fn to_tuple(self) -> (String,String) {
            (self.first_name, self.last_name)
        }
    }
    let mut p = Person::new("John","Smith");
    println!("{:?}", p);
    p.set_first_name("Jane");
    println!("{:?}", p);
    println!("{:?}", p.to_tuple());

}

#[allow(dead_code,warnings, unused_variables, unused_assignments)]
fn shadowing() {
   
    let x = 5;
    let x = x + 1;
    {
        let x = x * 3;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

}

#[allow(dead_code,warnings, unused_variables, unused_assignments)]
fn dtypes() {

    let guess: u32 = "50".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("The value of floats is: {},{}", x,y);

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of x is: {},{},{}", five_hundred,six_point_four,one);

    //arrays
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    //println!("The value of a is: {}", a);
    
}

#[allow(dead_code,warnings, unused_variables,unused_assignments)]
fn cli() {

    use std::io;

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index less than 5.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

}

