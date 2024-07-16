fn main() {
    let s1: String = String::from("Hello World");
    //Borrowing value
    let s2 = &s1;

    println!("{s}", s=s1);
    println!("{s}", s=s2);
}