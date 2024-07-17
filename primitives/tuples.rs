fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

fn main() {
    let mut pair1 = (1, true);
    println!("Mutable pair {:?}", pair1);
    pair1.0 = 2;
    println!("Altered mutable pair {:?}", pair1);
    let reversed_pair = reverse(pair1);
    println!("Reversed pair {:?}", reversed_pair);
}