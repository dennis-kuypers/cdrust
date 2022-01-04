pub(crate) mod serialize {
    use serde::Serializer;

    pub(crate) fn mime_type<S>(mime_type: &mime::Mime, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = mime_type.to_string();
        serializer.serialize_str(&s)
    }
}

pub(crate) mod deserialize {
    use core::fmt;
    use serde::{de::Visitor, Deserializer};
    use std::str::FromStr;

    pub(crate) fn mime_type<'de, D>(deserializer: D) -> ::std::result::Result<mime::Mime, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MimeVisitor;

        impl<'a> Visitor<'a> for MimeVisitor {
            type Value = mime::Mime;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string with valid mime type")
            }

            fn visit_str<E>(self, s: &str) -> ::std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                mime::Mime::from_str(s).map_err(|_| serde::de::Error::custom("invalid mime type"))
            }
        }

        deserializer.deserialize_string(MimeVisitor)
    }
}
