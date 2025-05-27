use serde::{Deserialize, Serialize, de, ser::SerializeSeq};

#[derive(Debug, Default, Clone)]
pub struct Pos<T> {
    x: T,
    y: T,
}

impl<T> Serialize for Pos<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.x)?;
        seq.serialize_element(&self.y)?;
        seq.end()
    }
}

impl<'de, T> Deserialize<'de> for Pos<T>
where
    T: Deserialize<'de> + std::fmt::Debug + std::clone::Clone,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec: Vec<T> = Vec::deserialize(deserializer)?;
        let mut v = vec.iter();

        let (Some(x), Some(y), None) = (v.next(), v.next(), v.next()) else {
            return Err(de::Error::custom(format!(
                "Invalid Number of fields in Pos `{vec:?}`, expected 2, got {}",
                vec.len()
            )));
        };

        Ok(Pos {
            x: x.clone(),
            y: y.clone(),
        })
    }
}
