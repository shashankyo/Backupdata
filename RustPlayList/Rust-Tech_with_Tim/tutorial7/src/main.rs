fn main() {
    let cond = (2 as f32 ) <= 2.2;
    
    let cond2 = !(false ||  !cond);
    println!("{}", cond2);


    let food = "fruit";

    if food == "cookie"{
        println!("I like cookies too");
    }else if food == "fruit"{
        println!("That sounds healthy!")
    }else{
        println!("Oh, that's too bad!")
    }

}
