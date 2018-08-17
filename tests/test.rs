#[macro_use]
extern crate serde_derive;
extern crate serde_test;
#[macro_use]
extern crate serde_struct_wrapper;

#[cfg(test)]
mod tests {
    use serde_test::*;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[serde(remote = "Self")]
    struct Point {
        x: i32,
        y: i32,
    }
    deserialize_with_root!("point": Point);
    serialize_with_root!("point": Point);

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
}
