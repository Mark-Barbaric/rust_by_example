use std::fmt;

pub struct Structure(pub i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "struc params {}", self.0)
    }
}