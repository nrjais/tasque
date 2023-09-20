use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


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
  pub title: String,
  pub description: String,
  pub status: TaskStatus,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
