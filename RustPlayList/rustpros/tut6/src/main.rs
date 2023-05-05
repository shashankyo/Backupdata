// fn main() {
//     println!("Hello, world!");
//     test();
//     exam();
//     test();
//     add_numbers(20,30);
// }

// fn test(){
//     println!("Test has been called...");
// }

// fn exam(){
//     println!("exam is going place");
// }
// fn add_numbers(x:i32, y:i32){
//     println!("the sum is:{}", x+y)
// }

fn main(){
    println!("hello duniya");
    let result = add_numbers(12,3);
    println!("{}", result);

}

fn add_numbers(x:i32,y:i32)-> i32 {
   let result =  x+y;
   if result >10 {
    return result -10;
   }
   result
}