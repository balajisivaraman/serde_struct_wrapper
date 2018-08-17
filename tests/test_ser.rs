#[macro_use]
extern crate serde_derive;
extern crate serde_test;
#[macro_use]
extern crate serde_struct_wrapper;

use serde_test::*;

#[derive(Serialize, Debug, PartialEq)]
#[serde(remote = "Self")]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
serialize_with_root!("point": Point);

#[test]
fn serializes_struct_with_root() {
    let s = Point { x: 0, y: 0 };
    assert_ser_tokens(
        &s,
        &[
            Token::Struct {
                name: "Wrapper",
                len: 1,
            },
            Token::Str("point"),
            Token::Struct {
                name: "Point",
                len: 2,
            },
            Token::Str("x"),
            Token::I32(0),
            Token::Str("y"),
            Token::I32(0),
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}
