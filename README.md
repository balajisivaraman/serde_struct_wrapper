## Serde Struct Wrapper

[![Build
Status](https://travis-ci.org/balajisivaraman/serde_struct_wrapper.svg?branch=master)](https://travis-ci.org/balajisivaraman/serde_struct_wrapper)
[![Latest
Version](https://img.shields.io/crates/v/serde_struct_wrapper.svg)](https://crates.io/crates/serde_struct_wrapper)
[![Rust
Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/serde_struct_wrapper/)

This crate provides macros that enable wrapping Rust structs with
alternate root keys during serialization and deserialization using
Serde. In principle, it offers a functionality similar to the
`@JsonRootName` annotation for Java's
[Jackson](https://github.com/FasterXML/jackson-annotations/wiki/Jackson-Annotations#serialization-details)
framework.

Note that this crate is primarily intended to be used in conjunction
with the [`serde_json`](https://crates.io/crates/serde_json) crate. It
has not been tested with other data formats.

## Usage

Add this to your Cargo.toml:

```toml
serde_struct_wrapper = "0.3"
```

You can use the `serde_with_root!` macro as shown below to both
serialize and deserialize a Struct with an alternate root key. (Please
note the use of the `#[serde(remote = "Self")]` attribute on the
Struct letting SerDe know of the alernate `Serialize` and
`Deserialize` implementations provided by the macro.)

```rust
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_struct_wrapper;
#[derive(Serialize, Deserialize, Debug)]
#[serde(remote = "Self")]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
serde_with_root!("point": Point);
```

The above will let you serialize/deserialize a JSON structure like the
following:

```json
{
    "point": {
        "x": 1,
        "y": 2
    }
}
```

For getting only the `Serializer` implementation, use the
`serialize_with_root!` macro; likewise with the
`deserialize_with_root!` macro for only the `Deserializer`
implementation.

## License

Serde is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
http://opensource.org/licenses/MIT)

at your option.

## Credits

The initial `Deserializer` implementation for this crate was provided by David
Tolnay in [this](https://github.com/serde-rs/serde/issues/1345) Github issue.
The code provided there was used as the base to provide this crate.
