use serde::{Deserialize, Serialize};
use crate::task::Status;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    id: u64,
    description: String,
    status: Status,
    // TODO: created_at
    // TODO: completed_at
}
