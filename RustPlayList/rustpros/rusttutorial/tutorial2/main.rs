// fn main(){

//     const SECONDS_IN_MINUTE: u32 = 60;
//     //  SECONDS_IN_MINUTE = 100
//     println!("{}", SECONDS_IN_MINUTE)
// }

// fn main(){
//     println!("Hello, World");

//     // let x = 2; 
// }

fn main(){
    let mut tup : (i32,bool,char) = (4,true,'k');
    
    // tup.0 = 10;
    // tup.0  = 12;
    // tup.0 = 11;
    // tup.0  = 2;
    // tup.0 = 14;
    // tup.0  = 19;
    tup = (10,false,'a');
    println!("{}",tup.1);
    println!("{}",tup.0);
    println!("{}",tup.2);

}