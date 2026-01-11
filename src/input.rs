use std::cmp::min;
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::slice::Chunks;

use serde::Deserialize;
use serde::Serialize;
use serde::de::MapAccess;
use serde::de::Visitor;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Ordermap(Vec<(String, String)>);

pub fn load_input(filename: &OsStr) -> Ordermap {
    let contents = fs::read_to_string(filename).expect("Failed to read input file");

    let data: Ordermap = toml::from_str(&contents).expect("Failed to parse TOML from input file");

    data
}

impl Ordermap {
    pub fn new(data: Vec<(String, String)>) -> Self {
        Self(data)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.0.iter().map(|x| (&x.0, &x.1))
    }

    pub fn chunks(&self, chunk_size: usize) -> Chunks<'_, (String, String)> {
        self.0.chunks(chunk_size)
    }
}

struct OrdermapVisitor;

impl<'de> Visitor<'de> for OrdermapVisitor {
    type Value = Ordermap;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map")
    }

    #[inline]
    fn visit_map<T>(self, mut access: T) -> Result<Ordermap, T::Error>
    where
        T: MapAccess<'de>,
    {
        let mut values = Vec::with_capacity(min(access.size_hint().unwrap_or(0), 4096));

        while let Some((key, value)) = access.next_entry()? {
            values.push((key, value));
        }

        Ok(Ordermap(values))
    }
}

impl<'de> Deserialize<'de> for Ordermap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(OrdermapVisitor)
    }
}

impl Serialize for Ordermap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_map(self.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordermap_serialization_preserves_order() {
        let data = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key3".to_string(), "value3".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ];
        let ordermap = Ordermap::new(data);

        let actual = toml::to_string(&ordermap).expect("Failed to serialize");

        let expected = r#"key1 = "value1"
key3 = "value3"
key2 = "value2"
"#;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_ordermap_deserialization_preserves_order() {
        let toml_str = r#"key1 = "value1"
key3 = "value3"
key2 = "value2"
"#;
        let actual: Ordermap = toml::from_str(toml_str).expect("Failed to deserialize");

        let expected = Ordermap::new(vec![
            ("key1".to_string(), "value1".to_string()),
            ("key3".to_string(), "value3".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ]);
        assert_eq!(expected, actual);
    }
}
