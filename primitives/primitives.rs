fn main() {
    let _logical: bool = true;

    let _a_float: f64 = 1.0;
    let _an_integer = 5i32;

    let _a_float2 = 1.0f32;
    
    let mut _mutable_signed = 134i16;
    println!("Mutable signed before {x}", x=_mutable_signed);
    _mutable_signed = 132;
    println!("Mutable signed after {x}", x=_mutable_signed);
}