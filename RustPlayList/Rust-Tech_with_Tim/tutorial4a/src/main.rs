fn main() {
   let mut tup: (i32, bool, char) = (1, true, '5');
   tup.0 = 10;
   println!("{}", tup.0);
}
