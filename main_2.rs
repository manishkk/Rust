fn main_2() {
   
  let x = 60; //i32
  let y: i64 = 70; //i64
  let z: u64 = 80; //u64 unsigned
  let b: bool = false; //Boolean values
  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
  println!("The value of z is: {}", z);
  println!("The value of b is: {}", b);
  
  let heart_eyed_cat = '😻';
  println!("The value is: {}", heart_eyed_cat);
   
  let tup = (500, 6.4, 1);
  let (p, _q, _r) = tup; //underscore because q and r are unused variables
  println!("The value of p is: {}", p); 
   
  let a = [1, 2, 3, 4, 5];
  let first = a[0];
  let second = a[1];
  println!("The value of first is: {}", first); 
  println!("The value of second is: {}", second); 
  // Here discussed about different types of data types
   
}
