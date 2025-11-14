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

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> //Simple Tag, un-owned str
                where
                E: serde::de::Error,
            {
                Ok(Tag::Simple(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> //Simple Tag, owned String? unsure if needed
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
                if seq.next_element::<serde::de::IgnoredAny>()?
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


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn serialize_from_tag_to_str() {
        // HashMap needed because TOML cannot be represented purely as a string / array
        let wrapper = HashMap::from( [( "value".to_string(), Tag::Simple("TODO".to_string()) )] );
        let tag_single_str = toml::to_string(&wrapper).unwrap();
        assert_eq!(tag_single_str.trim(), "value = \"TODO\"".to_string());

        let wrapper = HashMap::from( [( "value".to_string(), Tag::KV("Due".to_string(), "Today".to_string()) )] );
        let tag_kv_str = toml::to_string(&wrapper).unwrap();
        assert_eq!(tag_kv_str.trim(), "value = [\"Due\", \"Today\"]".to_string());
    }

    #[test]
    fn deserialize_from_str_to_tag() {
        #[derive(serde::Deserialize)]
        struct Wrapper {
            tag: Tag
        }

        let simple_tag_str = "tag = \"Phase\"";
        let simple_tag_wrapper: Wrapper = toml::from_str::<Wrapper>(simple_tag_str).unwrap();
        assert_eq!(simple_tag_wrapper.tag, Tag::Simple("Phase".to_string()));

        let kv_tag_str = "tag = [\"Duck\",\"Quack\"]";
        let kv_tag_wrapper: Wrapper = toml::from_str::<Wrapper>(kv_tag_str).unwrap();
        assert_eq!(kv_tag_wrapper.tag, Tag::KV("Duck".to_string(), "Quack".to_string()));
    }
}