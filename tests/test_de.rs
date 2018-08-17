#[macro_use]
extern crate serde_derive;
extern crate serde_test;
#[macro_use]
extern crate serde_struct_wrapper;

use serde_test::*;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(remote = "Self")]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
deserialize_with_root!("point": Point);

#[test]
fn deserializes_struct_with_root() {
    let s = Point { x: 0, y: 0 };
    assert_de_tokens(
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

#[test]
fn deserializer_throws_error_on_duplicate_root() {
    assert_de_tokens_error::<Point>(
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
            Token::Str("point"),
            Token::StructEnd,
        ],
        "duplicate field `point`",
    );
}

#[test]
fn deserializer_throws_error_on_missing_root() {
    assert_de_tokens_error::<Point>(
        &[
            Token::Struct {
                name: "Wrapper",
                len: 2,
            },
            Token::Str("x"),
            Token::I32(0),
            Token::Str("y"),
            Token::I32(0),
            Token::StructEnd,
        ],
        "missing field `point`",
    );
}
