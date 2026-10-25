mod device;
mod data;
use crate::data::Data;
fn main() {
    let test: Data = Data {
        message: vec!(1, 5, 6),
    };
    println!("{}",test.size());
}
