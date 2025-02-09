fn main() {
    another_function();
    function_parameters(5);
    print_label_measurement(9, 'm');
    let num = function_return();
    println!("num = {num}");
    let num = function_with_explicit_return();
    println!("num = {num}");
}

fn another_function() {
    println!("another function!");
}

fn function_parameters(x: i32) {
    println!("the value of x is {x}");
}

fn print_label_measurement(value: i32, unit_measurement: char) {
    println!("the measurement is {value}{unit_measurement}")
}

fn function_return() -> i32 {
    5
}

fn function_with_explicit_return() -> i32 {
    return 7;
}