//use rand::{distributions::Uniform, distributions::Standard,Rng}; // 0.6.5

fn main() {
    println!("Hello, Parallel World!");
    
    let a: [u64; 5] = [1, 2, 3, 4, 5];
    println!("Array is: {:?}", a);
    let s = sum_of_squares(&a);
    println!("Sum Of Squares of Array=> {:?}", s);

    //let my_array: [u64; 8] = rand::random();
    //println!("{:?}", my_array);

    sos_functional();

    //rand::thread_rng().gen_range(1,7)
}

fn sum_of_squares(input: &[u64]) -> u64 {
    input.iter()
         .map(|&i| i * i*i)
         .sum()
}

#[allow(dead_code,warnings, unused_variables, unused_assignments)]
fn sos_functional() {
    use rand::{distributions::Uniform, distributions::Standard,Rng}; // 0.6.5
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 20);
    let data: Vec<u64> = (0..10).map(|_| rng.sample(&range)).collect();
    println!("Random Vector=>{:?}", data);
    let sos = data.iter().map(|x| x.pow(2)).sum::<u64>();
    println!("#*****SOS=>{:?}", sos);
}