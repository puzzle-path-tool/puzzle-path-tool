use serde::{Deserialize, Serialize, de, ser::SerializeSeq};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum NegativeFlag {
    #[serde(rename = "ratio")]
    Ratio,
    #[serde(rename = "xv")]
    XV,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub struct Negative {
    ratio: bool,
    xv: bool,
}

impl Serialize for Negative {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut flags = vec![];
        if self.ratio {
            flags.push(NegativeFlag::Ratio);
        }
        if self.xv {
            flags.push(NegativeFlag::XV);
        }

        let mut seq = serializer.serialize_seq(Some(flags.len()))?;
        for flag in flags {
            seq.serialize_element(&flag)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Negative {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let flags: Vec<NegativeFlag> = Vec::deserialize(deserializer)?;

        let mut negatives = Negative::default();

        for flag in flags {
            match flag {
                NegativeFlag::Ratio => negatives.ratio = true,
                NegativeFlag::XV => negatives.xv = true,
            }
        }

        Ok(negatives)
    }
}
