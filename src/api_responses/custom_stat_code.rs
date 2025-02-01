use reqwest::StatusCode;

#[derive(Debug)]
pub struct StatCode(StatusCode);

impl Default for StatCode {
    fn default() -> Self {
        StatCode(StatusCode::NOT_FOUND)
    }
}

impl<'de> serde::Deserialize<'de> for StatCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        struct StatCodeVisitor;
        impl<'de> serde::de::Visitor<'de> for StatCodeVisitor {
            type Value = StatCode;
            
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expecting a non-zero u16 number.")
            }

        }
        deserializer.deserialize_u16(StatCodeVisitor)
    }
}

impl serde::Serialize for StatCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_u16(self.0.as_u16())
    }
}

impl AsRef<StatusCode> for StatCode {
    fn as_ref(&self) -> &StatusCode {
        &self.0
    }
}

impl From<StatusCode> for StatCode {
    fn from(code: StatusCode) -> Self {
        StatCode(code)
    }
}
