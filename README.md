## Serde Struct Wrapper

This crate provides macros that enable wrapping Rust structs with alternate root
keys during serialization and deserialization using Serde. In principle, it
offers a functionality similar to the `@JsonRoot` annotation for Java's
[Jackson](https://github.com/FasterXML/jackson-annotations/wiki/Jackson-Annotations#serialization-details)
framework.

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
