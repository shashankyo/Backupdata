// fn main() {
//     let x: u8 = 12;
//     let y: u8 = 10;

//     let z = x + y;
//     println!("{}", z);

//     let x: f64 = 120.0;
//     let y: f64 = 10.0;

//     let z = x / y;
//     println!("{}", z);

//     let x= (i32::MAX as i64) + 1;
//     let y=10_i32;

//     let z = x as i32 / y;
//     println!("{}", z);

    use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("expected to read line");

    let int_input: i64 = input.trim().parse().unwrap();

    println!("{}", int_input + 2)
}