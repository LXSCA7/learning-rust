fn main() {
    // mutability:

    // // let x: i32 = 5;
    // let mut x: i32 = 5;
    // println!("the value of x is: {x}");
    // // if x isnt mutable, we cant compile.
    // x = 6;
    // println!("the value of x is: {x}");

    // constants:
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);

    // shadowing:

    let x: i32 = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("the value of x in the inner the scope is: {x}");
    }
    println!("the value of x is: {x}");

}
