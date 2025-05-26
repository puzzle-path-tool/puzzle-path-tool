use serde::{Deserialize, Serialize, de};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Region {
    InRegion(i32),
    NoRegion,
}

impl Serialize for Region {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Region::InRegion(n) => serializer.serialize_i32(*n),
            Region::NoRegion => serializer.serialize_none(),
        }
    }
}

impl<'de> Deserialize<'de> for Region {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let val: Option<i32> = Deserialize::deserialize(deserializer)?;
        let region = match val {
            Some(n) => Region::InRegion(n),
            None => Region::NoRegion,
        };
        Ok(region)
    }
}
