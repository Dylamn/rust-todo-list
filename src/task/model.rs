use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::task::Status;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    id: u64,
    description: String,
    status: Status,
    created_at: DateTime<Utc>,
    completed_at: DateTime<Utc>
}
