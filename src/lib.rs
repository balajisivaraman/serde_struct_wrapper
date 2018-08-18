//! This crate provides macros that enable wrapping Rust structs with
//! alternate root keys during serialization and deserialization using
//! Serde. In principle, it offers a functionality similar to the
//! `@JsonRootName` annotation for Java's
//! [Jackson](https://github.com/FasterXML/jackson-annotations/wiki/Jackson-Annotations#serialization-details)
//! framework.
//!
//! Note that this crate is primarily intended to be used in conjunction
//! with the [`serde_json`](https://crates.io/crates/serde_json) crate. It
//! has not been tested with other data formats.
//!
//! ## Usage
//!
//! Add this to your Cargo.toml:
//!
//! ```toml
//! serde_struct_wrapper = "0.3"
//! ```
//!
//! You can use the `serde_with_root!` macro as shown below to both
//! serialize and deserialize a Struct with an alternate root key. (Please
//! note the use of the `#[serde(remote = "Self")]` attribute on the
//! Struct letting SerDe know of the alernate `Serialize` and
//! `Deserialize` implementations provided by the macro.)
//!
//! ```rust
//! extern crate serde;
//! #[macro_use]
//! extern crate serde_derive;
//! #[macro_use]
//! extern crate serde_struct_wrapper;
//! #[derive(Serialize, Deserialize, Debug)]
//! #[serde(remote = "Self")]
//! pub struct Point {
//!     pub x: i32,
//!     pub y: i32,
//! }
//! serde_with_root!("point": Point);
//! # fn main() {}
//! ```
//!
//! The above will let you serialize/deserialize a JSON structure like the
//! following:
//!
//! ```json
//! {
//!     "point": {
//!         "x": 1,
//!         "y": 2
//!     }
//! }
//! ```
//!
//! For getting only the `Serializer` implementation, use the
//! `serialize_with_root!` macro; likewise with the
//! `deserialize_with_root!` macro for only the `Deserializer`
//! implementation.

#[doc(hidden)]
pub extern crate serde;

#[doc(hidden)]
pub extern crate core;

/// Generates a custom SerDe `Deseralize` implementation that adds an
/// alternate root key to a Struct during deserialization.

/// # Example
///
/// ```rust
/// extern crate serde;
/// #[macro_use]
/// extern crate serde_derive;
/// #[macro_use]
/// extern crate serde_struct_wrapper;
/// #[derive(Deserialize)]
/// #[serde(remote = "Self")]
/// pub struct Point {
///     pub x: i32,
///     pub y: i32,
/// }
/// deserialize_with_root!("point": Point);
/// # fn main() {}
/// ```
///
/// The above will deserialize a JSON structure like the following:
///
/// ```json
/// {
///     "point": {
///         "x": 1,
///         "y": 2
///     }
/// }
/// ```
#[macro_export]
macro_rules! deserialize_with_root {
    ($root:tt : $inner:ty) => {
        impl<'de> $crate::serde::Deserialize<'de> for $inner {
            fn deserialize<D>(deserializer: D) -> $crate::core::result::Result<Self, D::Error>
            where
                D: $crate::serde::Deserializer<'de>,
            {
                #[derive(Deserialize)]
                #[serde(field_identifier)]
                enum RootKey {
                    #[serde(rename = $root)]
                    Root,
                    #[serde(other)]
                    Other,
                }

                struct WrapperVisitor;

                impl<'de> $crate::serde::de::DeserializeSeed<'de> for WrapperVisitor {
                    type Value = $inner;

                    fn deserialize<D>(
                        self,
                        deserializer: D,
                    ) -> $crate::core::result::Result<Self::Value, D::Error>
                    where
                        D: $crate::serde::Deserializer<'de>,
                    {
                        <$inner>::deserialize(deserializer)
                    }
                }

                impl<'de> $crate::serde::de::Visitor<'de> for WrapperVisitor {
                    type Value = $inner;

                    fn expecting(
                        &self,
                        formatter: &mut $crate::core::fmt::Formatter,
                    ) -> $crate::core::fmt::Result {
                        formatter.write_str(concat!("a wrapper around ", stringify!($inner)))
                    }

                    fn visit_map<A>(
                        self,
                        mut map: A,
                    ) -> $crate::core::result::Result<Self::Value, A::Error>
                    where
                        A: $crate::serde::de::MapAccess<'de>,
                    {
                        let mut inner = $crate::core::option::Option::None;
                        while let $crate::core::option::Option::Some(key) = map.next_key()? {
                            match key {
                                RootKey::Root => {
                                    if inner.is_some() {
                                        return Err($crate::serde::de::Error::duplicate_field(
                                            $root,
                                        ));
                                    }
                                    inner = Some(map.next_value_seed(WrapperVisitor)?);
                                }
                                RootKey::Other => {
                                    map.next_value::<$crate::serde::de::IgnoredAny>()?;
                                }
                            }
                        }
                        inner.ok_or_else(|| $crate::serde::de::Error::missing_field($root))
                    }
                }

                deserializer.deserialize_struct("Wrapper", &[$root], WrapperVisitor)
            }
        }
    };
}

/// Generates a custom SerDe `Seralize` implementation that adds an
/// alternate root key to a Struct during serialization.

/// # Example
///
/// ```rust
/// extern crate serde;
/// #[macro_use]
/// extern crate serde_derive;
/// #[macro_use]
/// extern crate serde_struct_wrapper;
/// #[derive(Serialize)]
/// #[serde(remote = "Self")]
/// pub struct Point {
///     pub x: i32,
///     pub y: i32,
/// }
/// serialize_with_root!("point": Point);
/// # fn main() {}
/// ```
///
/// The above will serialize a JSON structure like the following:
///
/// ```json
/// {
///     "point": {
///         "x": 1,
///         "y": 2
///     }
/// }
/// ```
#[macro_export]
macro_rules! serialize_with_root {
    ($root:tt : $inner:ty) => {
        use $crate::core::result::Result;
        use $crate::serde::ser::{Serialize, SerializeStruct, Serializer};

        impl Serialize for $inner {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::serde::ser::Serializer,
            {
                struct Wrapper<'a> {
                    root: &'a $inner,
                }

                impl<'a> $crate::serde::Serialize for Wrapper<'a> {
                    fn serialize<S>(
                        &self,
                        serializer: S,
                    ) -> $crate::core::result::Result<S::Ok, S::Error>
                    where
                        S: $crate::serde::Serializer,
                    {
                        <$inner>::serialize(&self.root, serializer)
                    }
                }

                let mut state = serializer.serialize_struct("Wrapper", 1)?;
                state.serialize_field($root, &Wrapper { root: self });
                state.end()
            }
        }
    };
}

/// Helper macro that will generate both the `Serialize` and
/// `Deserialize` implementations with an alternate root key.

/// This is the same as manually calling both `deserialize_with_root!`
/// and `serialize_with_root!`.
#[macro_export]
macro_rules! serde_with_root {
    ($root:tt : $inner:ty) => {
        deserialize_with_root!($root: $inner);
        serialize_with_root!($root: $inner);
    };
}
