fn main_7() {
    another_function(5, 6);

    let z = {
        let p = 3;
        p + 1
    };

    println!("The value of z is: {}", z);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
