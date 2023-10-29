use core::fmt;
use std::fmt::Display;
use std::str::FromStr;

use indexmap::IndexMap;

pub struct QueryString(IndexMap<String, String>);

impl QueryString {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|v| v.as_str())
    }

    pub fn get_value<T: FromStr>(&self, key: &str) -> Option<T> {
        match self.0.get(key) {
            Some(value) => T::from_str(value).ok(),
            None => None,
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }

    pub fn set_value<T: Display>(&mut self, key: &str, value: &T) {
        self.0.insert(key.to_string(), value.to_string());
    }

    pub fn parse(input: &str) -> Self {
        let mut map = IndexMap::new();
        for pair in input.split('&') {
            let mut it = pair.split('=').take(2);
            let (Some(key), Some(value)) = (it.next(), it.next()) else {
          continue;
        };
            map.insert(key.to_string(), value.to_string());
        }
        Self(map)
    }
}

impl fmt::Display for QueryString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut it = self.0.iter();

        if let Some((key, value)) = it.next() {
            write!(f, "{key}={value}")?;
        }

        for (key, value) in it {
            write!(f, "&{key}={value}")?;
        }
        Ok(())
    }
}
