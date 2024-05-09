use crate::domain::clip::ClipError;
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, From)]
pub struct ShortCode(String);

impl ShortCode {
    pub fn new() -> Self {
        use rand::prelude::*;
        let mut rng = thread_rng();
        let allowed_chars = ['a', 'b', 'c', 'd', '1', '2', '3', '4'];

        let mut shortcode = String::with_capacity(10);

        for _ in 0..10 {
            shortcode.push(
                *allowed_chars
                    .choose(&mut rng)
                    .expect("sampling array should have values"),
            );
        }

        Self(shortcode)
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Default for ShortCode {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ShortCode> for String {
    fn from(value: ShortCode) -> Self {
        value.0
    }
}

impl From<&str> for ShortCode {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl FromStr for ShortCode {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}
