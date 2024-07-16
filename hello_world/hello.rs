mod formatted_print;

fn main() {
    let x = 5 + 95;
    println!("Is `x` 10 or 100? x = {}", x);
    println!("Hello world");
    formatted_print::print_test();
    formatted_print::print_with_padding();
    formatted_print::print_arguments();
}