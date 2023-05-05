fn main() {
    let x  = 2;
    let y  = x;

    println!("{}", y);

    example();
    //When the control come here the alloted value of a and b in stack is removed 
}
//When control comes here main ends so x and y values also be removed
fn example(){
    let a  = "true";
    let b = false;
}

let x  = 2;
let String = String::from("hello");