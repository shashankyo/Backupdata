fn main() {
    println!("Hello, world!");
    test();
    test();
    test_one();
    test_two();
    add_numbers(20, 30);
    let result = add_numbers1(2,3);
    println!("{}", result);

    //Statement inside a expression is allowed
    //But expression inside a statement is not allowed
    let number = {
        let x = 3;
        x + 1
    };
    println!("{}", number);
}

fn test(){
    println!("Test has been called...")
}
fn test_one(){
    println!("Test one has been called...")
}
fn test_two(){
    println!("Test two has been called...")
}

fn add_numbers(x: i32, y: i32){
    println!("The sum is: {}", x + y)
}

//-> is the return type i32
fn add_numbers1(x: i32, y: i32) -> i32{
    // x + y
    1000
}