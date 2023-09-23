use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::{Uuid, Timestamp, timestamp::context};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TaskResult {
  pub result: String,
  pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum TaskStatus {
  Pending,
  InProgress,
  Completed(TaskResult),
  Canceled,
  Failed,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Task {
  pub id: Uuid,
  pub name: String,
  pub data: Vec<u8>,
  pub status: TaskStatus,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Task {
  pub fn new(name: String, data: Vec<u8>) -> Self {
    Self {
      id: Uuid::new_v7(Timestamp::now(context::NoContext)),
      name,
      data,
      status: TaskStatus::Pending,
      created_at: Utc::now(),
      updated_at: Utc::now(),
    }
  }
}
