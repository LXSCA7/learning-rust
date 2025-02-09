fn main() {
    let number: i16 = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    
    if number != 0 {
        println!("number != 0");
    }

    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("the value of number is: {number}");

    let mut iterations = 0;

    while iterations != 4 {
        println!("loop!");
        iterations += 1;
    }

    for i in 0..4 {
        println!("{}", i);
    }
}
