use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogQueryArgs {
    pub until: DateTime<Utc>,
    pub limit: usize,
}
