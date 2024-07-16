fn print_test() {
    let x = 68200;
    println!("Base 10: {}", x);
    println!("Base 2 (binary): {:b}", x);
}

fn print_with_padding() {
    let x = 1;
    println!("X with left padding: {number:0>5}", number=x);
    println!("X with right padding: {number:0<5}", number=x);
}

fn print_arguments() {
    let first_name = "Mark";
    let second_name = "Barbaric";
    println!("My name is {second_name}, {first_name} {second_name}", first_name=first_name, second_name=second_name);
}

#[allow(dead_code)]
fn dead_method() {
}

fn main() {
    print_test();
    print_with_padding();
    print_arguments();
}