use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} hello", self.0)
    }
}

fn main() {
    println!("{}",Structure(12));
}
