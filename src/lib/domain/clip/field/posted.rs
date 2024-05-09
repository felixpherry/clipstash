use crate::domain::time::Time;
use chrono::Utc;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Constructor)]
pub struct Posted(Time);

impl Posted {
    pub fn into_inner(self) -> Time {
        self.0
    }
}

impl Default for Posted {
    fn default() -> Self {
        Self::new(Utc::now().into())
    }
}
