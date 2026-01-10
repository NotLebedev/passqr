use std::cmp::min;
use std::env;
use std::fmt;
use std::fs;

use serde::Deserialize;
use serde::Serialize;
use serde::de::MapAccess;
use serde::de::Visitor;

pub struct Ordermap(Vec<(String, String)>);

pub fn load_input() -> Ordermap {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file.toml>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Failed to read input file");

    let data: Ordermap = toml::from_str(&contents).expect("Failed to parse TOML from input file");

    data
}

impl Ordermap {
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.0.iter().map(|x| (&x.0, &x.1))
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
