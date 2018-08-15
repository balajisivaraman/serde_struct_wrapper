#[doc(hidden)]
pub extern crate serde;

#[doc(hidden)]
pub extern crate core;

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

                    fn deserialize<D>(self, deserializer: D) -> $crate::core::result::Result<Self::Value, D::Error>
                    where
                        D: $crate::serde::Deserializer<'de>,
                    {
                        <$inner>::deserialize(deserializer)
                    }
                }

                impl<'de> $crate::serde::de::Visitor<'de> for WrapperVisitor {
                    type Value = $inner;

                    fn expecting(&self, formatter: &mut $crate::core::fmt::Formatter) -> $crate::core::fmt::Result {
                        formatter.write_str(concat!("a wrapper around ", stringify!($inner)))
                    }

                    fn visit_map<A>(self, mut map: A) -> $crate::core::result::Result<Self::Value, A::Error>
                    where
                        A: $crate::serde::de::MapAccess<'de>,
                    {
                        let mut inner = $crate::core::option::Option::None;
                        while let $crate::core::option::Option::Some(key) = map.next_key()? {
                            match key {
                                RootKey::Root => {
                                    if inner.is_some() {
                                        return Err($crate::serde::de::Error::duplicate_field($root));
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
    }
}
