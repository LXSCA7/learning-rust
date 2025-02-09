fn main() {
    // let guess: = "42".parse().expect("not a number.");
    //     ^^^^^         ----- type must be known at this point

    let guess: u32 = "42".parse().expect("not a number.");
    println!("{}", guess);
    
    let x = 2.0; // f64 (default)
    let y: f32 = 3.0; // f32
    println!("{}", x);
    println!("{}", y);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("first tuple value is: {}", tup.0);

    let a = [1, 2, 3, 4, 5, 6, 7];
    let months = ["january", "february", "march"];
    let first_month: &str = months[0];
    let first_int = a[0];
}
