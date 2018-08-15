#[macro_use]
extern crate serde_derive;
extern crate serde_test;
#[macro_use]
extern crate serde_struct_wrapper;

#[cfg(test)]
mod tests {
    use serde_test::{Token, assert_de_tokens};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[serde(remote="Self")]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn deserializes_struct_with_root() {

        deserialize_with_root!("point": Point);

        let s = Point {
            x: 0,
            y: 0
        };
        assert_de_tokens(&s, &[
                      Token::Struct {
                          name: "Wrapper",
                          len: 1
                      },
                      Token::Str("point"),
                      Token::Struct {
                          name: "Point",
                          len: 2
                      },
                      Token::Str("x"),
                      Token::I32(0),
                      Token::Str("y"),
                      Token::I32(0),
                      Token::StructEnd,
                      Token::StructEnd,
        ]);
   }
}
