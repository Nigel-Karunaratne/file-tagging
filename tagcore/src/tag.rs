use serde::{Serialize, Deserialize};

/// Enum for a Tag, which can either be a single String (Simple) or a 2-tuple of Strings (KV)
#[derive(Debug)]
pub enum Tag {
	Simple(String),
	KV(String, String)
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Simple(left), Self::Simple(right)) => left == right,
            (Self::KV(l0, l1), Self::KV(r0, r1)) => l0 == r0 && l1 == r1,
            _ => false,
        }
    }
}

/// Custom deserialization for Tag Struct, so that the desired output can be created
impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        
        struct TagVisitor;
        impl<'de> serde::de::Visitor<'de> for TagVisitor {
            type Value = Tag;
        
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result { //Error msg
                formatter.write_str("a String or 2-elem tuple of Strings")
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> //Simple Tag
                where
                    E: serde::de::Error, {
                Ok(Tag::Simple(v.to_string()))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> //Key-Value tag
                where
                    A: serde::de::SeqAccess<'de>, {
                let first: String = seq.next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &Self))?;
                let second: String = seq.next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &Self))?;
                if seq.next_element::<String>()?
                    .is_some() {
                        return Err(serde::de::Error::invalid_length(3, &self));
                }

                Ok(Tag::KV(first, second))
            }
        }

        deserializer.deserialize_any(TagVisitor)
    }
}

/// Custom serialization for Tag Struct, so that the desired output can be created
impl Serialize for Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            Tag::Simple(value) => serializer.serialize_str(value),
            Tag::KV(key,value) => {
                let pair = [&key[..], &value[..]];
                pair.serialize(serializer)
            }
        }
    }
}
