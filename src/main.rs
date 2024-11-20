use pysmennyi_phone_parser::*;
fn main() {
    let input = "00 380 (50) 123 4567";
    let phones = Phone::from_string(input).unwrap();
    println!("{:?}", phones);
}
