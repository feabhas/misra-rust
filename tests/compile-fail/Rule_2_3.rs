#[deny(dead_code)]

fn main() {
    type LocalType = i16; //~ ERROR type alias is never used: `LocalType`
}
